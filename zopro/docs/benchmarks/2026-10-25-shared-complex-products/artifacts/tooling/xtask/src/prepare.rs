//! Copy an audited source snapshot and apply explicit candidate files.
use crate::{
    process::{json, sha256},
    Result,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Component, Path, PathBuf},
};

#[derive(Deserialize)]
struct Manifest {
    source_files_sha256: BTreeMap<String, String>,
}

#[derive(Serialize)]
pub struct Snapshot {
    pub workspace: PathBuf,
    pub base_manifest_sha256: String,
    pub base_files: usize,
    pub overlay_files: BTreeMap<String, String>,
    pub source_pins: BTreeMap<String, String>,
}

fn relative(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty()
        || path
            .components()
            .any(|p| !matches!(p, Component::Normal(_)))
        || path.starts_with(".xtask")
    {
        return Err("snapshot paths must be relative normal components".into());
    }
    Ok(())
}

fn collect(root: &Path, path: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        if kind.is_symlink() {
            return Err("candidate overlays cannot contain symlinks".into());
        }
        if kind.is_dir() {
            collect(root, &entry.path(), files)?;
        } else if kind.is_file() {
            let name = entry.path().strip_prefix(root)?.to_path_buf();
            relative(&name)?;
            files.push(name);
        } else {
            return Err("candidate overlay contains a non-file entry".into());
        }
    }
    Ok(())
}

pub fn prepare(
    manifest: &Path,
    root: &Path,
    destination: &Path,
    overlay: Option<&Path>,
) -> Result<Snapshot> {
    let manifest_hash = sha256(manifest)?;
    let manifest: Manifest = serde_json::from_slice(&std::fs::read(manifest)?)?;
    if manifest.source_files_sha256.is_empty() {
        return Err("empty source snapshot".into());
    }
    let root = root.canonicalize()?;
    for (name, hash) in &manifest.source_files_sha256 {
        relative(Path::new(name))?;
        let source = root.join(name).canonicalize()?;
        if !source.starts_with(&root) || &sha256(source)? != hash {
            return Err(format!("source pin changed: {name}").into());
        }
    }
    let mut overlays = Vec::new();
    if let Some(overlay) = overlay {
        collect(overlay, overlay, &mut overlays)?;
        overlays.sort();
    }
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::create_dir(destination)?;
    for (name, hash) in &manifest.source_files_sha256 {
        let output = destination.join(name);
        std::fs::create_dir_all(output.parent().unwrap())?;
        std::fs::copy(root.join(name), &output)?;
        if &sha256(&output)? != hash {
            return Err("source changed during snapshot copy".into());
        }
    }
    let base_files = manifest.source_files_sha256.len();
    let mut source_pins = manifest.source_files_sha256;
    let mut overlay_files = BTreeMap::new();
    if let Some(overlay) = overlay {
        for name in overlays {
            let input = overlay.join(&name);
            let hash = sha256(&input)?;
            let output = destination.join(&name);
            std::fs::create_dir_all(output.parent().unwrap())?;
            std::fs::copy(input, &output)?;
            if sha256(&output)? != hash {
                return Err("overlay changed during snapshot copy".into());
            }
            let name = name.to_str().ok_or("non-UTF-8 snapshot path")?.to_owned();
            overlay_files.insert(name.clone(), hash.clone());
            source_pins.insert(name, hash);
        }
    }
    let snapshot = Snapshot {
        workspace: destination.to_path_buf(),
        base_manifest_sha256: manifest_hash,
        base_files,
        overlay_files,
        source_pins: source_pins
            .into_iter()
            .map(|(name, hash)| {
                Ok((
                    destination
                        .join(name)
                        .to_str()
                        .ok_or("non-UTF-8 snapshot path")?
                        .into(),
                    hash,
                ))
            })
            .collect::<Result<_>>()?,
    };
    std::fs::create_dir(destination.join(".xtask"))?;
    json(destination.join(".xtask/snapshot.json"), &snapshot)?;
    json(
        destination.join(".xtask/source-pins.json"),
        &snapshot.source_pins,
    )?;
    Ok(snapshot)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn traversal_and_reserved_metadata_are_rejected() {
        for name in ["../outside", "/absolute", ".xtask/pins.json", ""] {
            assert!(relative(Path::new(name)).is_err());
        }
        relative(Path::new("crates/core/src/lib.rs")).unwrap();
    }
    #[test]
    fn snapshot_checks_base_and_overlay_and_never_overwrites_destination() {
        let temp = std::env::temp_dir().join(format!("zopro-prepare-test-{}", std::process::id()));
        std::fs::create_dir(&temp).unwrap();
        let root = temp.join("source");
        let overlay = temp.join("overlay");
        std::fs::create_dir(&root).unwrap();
        std::fs::create_dir(&overlay).unwrap();
        std::fs::write(root.join("input"), b"original").unwrap();
        std::fs::write(overlay.join("input"), b"candidate").unwrap();
        std::fs::write(overlay.join("new"), b"new module").unwrap();
        let manifest = temp.join("manifest.json");
        json(&manifest,&serde_json::json!({"source_files_sha256":{"input":sha256(root.join("input")).unwrap()}})).unwrap();
        let output = temp.join("output");
        let snapshot = prepare(&manifest, &root, &output, Some(&overlay)).unwrap();
        assert_eq!(snapshot.source_pins.len(), 2);
        assert_eq!(snapshot.base_files, 1);
        assert_eq!(std::fs::read(output.join("input")).unwrap(), b"candidate");
        assert!(prepare(&manifest, &root, &output, None).is_err());
        std::fs::write(root.join("input"), b"changed").unwrap();
        assert!(prepare(&manifest, &root, &temp.join("bad"), None).is_err());
        assert!(!temp.join("bad").exists());
        std::fs::remove_dir_all(temp).unwrap();
    }
}
