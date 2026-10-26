use super::elf::Elf;
use crate::Result;
use std::collections::BTreeMap;

pub struct Instruction {
    pub assembly: String,
    pub mnemonic: String,
    pub family: &'static str,
    pub memory: &'static str,
}

pub fn assembly_class(assembly: &str) -> (String, &'static str, &'static str) {
    let mut words = assembly.split_whitespace();
    let mut mnemonic = words.next().unwrap_or("");
    while matches!(
        mnemonic,
        "lock"
            | "rep"
            | "repz"
            | "repnz"
            | "bnd"
            | "data16"
            | "addr32"
            | "cs"
            | "ds"
            | "es"
            | "ss"
            | "fs"
            | "gs"
    ) || mnemonic.starts_with("rex")
    {
        mnemonic = words.next().unwrap_or("");
    }
    let operands = words.collect::<Vec<_>>().join(" ");
    let operands = operands.split('#').next().unwrap_or("");
    let family = if mnemonic.starts_with("vpmadd52") {
        "ifma"
    } else if mnemonic.starts_with("vmov")
        || mnemonic.starts_with("vpmov")
        || (mnemonic.starts_with("mov") && operands.contains("%xmm"))
    {
        "vector_move"
    } else if ["vpadd", "vpsub", "vadd", "vsub"]
        .iter()
        .any(|p| mnemonic.starts_with(p))
    {
        "vector_add_sub"
    } else if ["vpsll", "vpsrl", "vpsra", "vprol", "vpror"]
        .iter()
        .any(|p| mnemonic.starts_with(p))
    {
        "vector_shift"
    } else if ["vpand", "vpxor", "vpor", "vand", "vxor", "vor"]
        .iter()
        .any(|p| mnemonic.starts_with(p))
    {
        "vector_logic"
    } else if ["vpcmp", "vptest", "vcmp"]
        .iter()
        .any(|p| mnemonic.starts_with(p))
    {
        "vector_compare"
    } else if [
        "vperm",
        "vshuf",
        "vpunpck",
        "vunpck",
        "vinsert",
        "vextract",
        "vpbroadcast",
        "vbroadcast",
    ]
    .iter()
    .any(|p| mnemonic.starts_with(p))
    {
        "vector_permute"
    } else if mnemonic.starts_with('v') {
        "vector_other"
    } else if mnemonic.starts_with('k') {
        "mask"
    } else if mnemonic.starts_with("lea") {
        "address_generation"
    } else if ["mul", "imul", "div", "idiv"]
        .iter()
        .any(|p| mnemonic.starts_with(p))
    {
        "scalar_multiply_divide"
    } else if ["mov", "push", "pop", "xchg"]
        .iter()
        .any(|p| mnemonic.starts_with(p))
    {
        "scalar_move"
    } else if ["add", "sub", "adc", "sbb", "inc", "dec", "neg"]
        .iter()
        .any(|p| mnemonic.starts_with(p))
    {
        "scalar_add_sub"
    } else if [
        "and", "or", "xor", "not", "shl", "shr", "sar", "sal", "rol", "ror", "shld", "shrd",
        "bzhi", "bextr", "tzcnt", "lzcnt", "bsr", "bsf",
    ]
    .iter()
    .any(|p| mnemonic.starts_with(p))
    {
        "scalar_logic_shift"
    } else if ["cmp", "test", "set", "cmov"]
        .iter()
        .any(|p| mnemonic.starts_with(p))
    {
        "scalar_compare_select"
    } else if mnemonic.starts_with('j')
        || ["call", "ret", "loop"]
            .iter()
            .any(|p| mnemonic.starts_with(p))
    {
        "control"
    } else {
        "other"
    };
    let memory = if mnemonic.starts_with("lea") {
        "address_only"
    } else if mnemonic.starts_with("nop") || mnemonic.starts_with("endbr") {
        "none"
    } else if mnemonic.starts_with("prefetch") {
        "prefetch"
    } else if operands.contains('(') {
        if operands.contains("%rsp") {
            "rsp_operand"
        } else if operands.contains("%rbp") {
            "rbp_operand"
        } else if operands.contains("%rip") {
            "rip_operand"
        } else {
            "other_memory_operand"
        }
    } else if operands.contains("%fs:") || operands.contains("%gs:") {
        "segment_operand"
    } else if ["call", "ret", "push", "pop"]
        .iter()
        .any(|p| mnemonic.starts_with(p))
    {
        "implicit_stack"
    } else {
        "none"
    };
    (mnemonic.into(), family, memory)
}

fn line(text: &str) -> Result<Option<(u64, Vec<u8>, String)>> {
    let Some((address, tail)) = text.split_once(':') else {
        return Ok(None);
    };
    let address = address.trim();
    if address.is_empty() || !address.chars().all(|c| c.is_ascii_hexdigit()) {
        return Ok(None);
    }
    let fields = tail
        .split('\t')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    if fields.len() != 2 {
        return Err(format!("expected byte-bearing one-line objdump output: {text}").into());
    }
    let bytes = fields[0]
        .split_whitespace()
        .map(|word| {
            if word.len() != 2 {
                return Err("invalid objdump byte".into());
            }
            Ok(u8::from_str_radix(word, 16)?)
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Some((
        u64::from_str_radix(address, 16)?,
        bytes,
        fields[1].into(),
    )))
}

pub fn instructions(text: &str, elf: &Elf<'_>) -> Result<BTreeMap<u64, Instruction>> {
    let mut result = BTreeMap::new();
    for text in text.lines() {
        if let Some((address, bytes, assembly)) = line(text)? {
            elf.bind(address, &bytes)?;
            let (mnemonic, family, memory) = assembly_class(&assembly);
            if result
                .insert(
                    address,
                    Instruction {
                        assembly,
                        mnemonic,
                        family,
                        memory,
                    },
                )
                .is_some()
            {
                return Err("duplicate objdump instruction address".into());
            }
        }
    }
    if result.is_empty() {
        return Err("empty disassembly".into());
    }
    Ok(result)
}

pub struct Symbol {
    pub start: u64,
    pub end: u64,
    pub name: String,
}
pub struct Symbols {
    entries: Vec<Symbol>,
    maximum_end: Vec<u64>,
}
impl Symbols {
    pub fn parse(text: &str) -> Result<Self> {
        let mut entries = Vec::new();
        for line in text.lines() {
            let f = line.split_whitespace().collect::<Vec<_>>();
            if f.len() < 4 || !matches!(f[2], "t" | "T" | "w" | "W") {
                continue;
            }
            let start = u64::from_str_radix(f[0], 16)?;
            let size = u64::from_str_radix(f[1], 16)?;
            if size == 0 {
                continue;
            }
            entries.push(Symbol {
                start,
                end: start.checked_add(size).ok_or("symbol extent overflow")?,
                name: f[3..].join(" "),
            });
        }
        entries.sort_by(|a, b| a.start.cmp(&b.start).then(a.name.cmp(&b.name)));
        let mut max = 0;
        let maximum_end = entries
            .iter()
            .map(|s| {
                max = max.max(s.end);
                max
            })
            .collect();
        if entries.is_empty() {
            return Err("empty nm code-symbol table".into());
        }
        Ok(Self {
            entries,
            maximum_end,
        })
    }
    pub fn names(&self, address: u64) -> Vec<&str> {
        let mut i = self.entries.partition_point(|s| s.start <= address);
        let mut found = Vec::new();
        while i > 0 && self.maximum_end[i - 1] > address {
            i -= 1;
            let s = &self.entries[i];
            if address < s.end {
                found.push(s.name.as_str());
            }
        }
        found.sort_unstable();
        found.dedup();
        found
    }
}

pub fn scope(names: &[&str]) -> &'static str {
    fn one(name: &str) -> &'static str {
        if name.starts_with("zopro_core::msm::stream_g1_impl::<") {
            "g1_stream"
        } else if name.starts_with("zopro_core::msm::g2_prefix::apply::<") {
            "g2_prefix"
        } else if name.starts_with("zopro_core::msm::reduce_layer::<zopro_bn254::curve::G1Config>")
        {
            "g1_reduce"
        } else if name.starts_with("zopro_core::msm::reduce_layer::<zopro_bn254::curve::G2Config>")
        {
            "g2_reduce"
        } else if name == "zopro_core::msm_affine::fused_kh::feed" {
            "fused_feed"
        } else if name.starts_with("zopro_core::msm::")
            || name.starts_with("zopro_core::msm_affine::")
        {
            "other_msm"
        } else {
            "other_executable"
        }
    }
    if names.is_empty() {
        return "unscoped_executable";
    }
    let first = one(names[0]);
    if names.iter().any(|n| one(n) != first) {
        "ambiguous_alias"
    } else {
        first
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn labels_bytes_and_operand_comments_stay_distinct() {
        assert!(line("00000000000e2a00 <a::b>:").unwrap().is_none());
        let (ip, b, a) = line(" e2a06:\t48 89 d1      \tmov %rdx,%rcx")
            .unwrap()
            .unwrap();
        assert_eq!(ip, 0xe2a06);
        assert_eq!(b, [0x48, 0x89, 0xd1]);
        assert_eq!(a, "mov %rdx,%rcx");
        assert!(line("1000:\t48 xx\tmov %rdx,%rcx").is_err());
        assert!(line("1000:\tmov %rdx,%rcx").is_err());
        assert_eq!(assembly_class("lea 0x10(%rsp),%rax").2, "address_only");
        assert_eq!(assembly_class("nopl (%rax,%rax,1)").2, "none");
        assert_eq!(
            assembly_class("mov (%rip),%rax # (%rsp) is comment").2,
            "rip_operand"
        );
        assert_eq!(assembly_class("lock addq $0,(%rsp)").1, "scalar_add_sub");
        assert_eq!(
            assembly_class("vpmadd52luq 0x10(%rsp),%zmm1,%zmm2").1,
            "ifma"
        );
        assert_eq!(
            assembly_class("vpmadd52luq 0x10(%rsp),%zmm1,%zmm2").2,
            "rsp_operand"
        );
        assert_eq!(assembly_class("mov 0x10(%rbp),%rax").2, "rbp_operand");
    }
    #[test]
    fn overlapping_symbols_aliases_gaps_and_end_boundaries_are_visible() {
        let s = Symbols::parse(
            "100 40 t outer\n110 10 t inner\n110 10 t alias\n150 0 t zero\n200 20 t next",
        )
        .unwrap();
        assert_eq!(s.names(0x115), ["alias", "inner", "outer"]);
        assert_eq!(s.names(0x125), ["outer"]);
        assert!(s.names(0x140).is_empty());
        assert!(s.names(0x180).is_empty());
        assert_eq!(s.names(0x200), ["next"]);
        assert!(s.names(0x220).is_empty());
        assert_eq!(
            scope(&["zopro_core::msm::g2_prefix::apply::<G2, true>", "other"]),
            "ambiguous_alias"
        );
    }
}
