use serde_json::json;
use std::{collections::BTreeMap, fs, path::Path};

fn text(s: &str, open: &str, close: &str) -> String {
    let a = s.find(open).unwrap() + open.len();
    let b = s[a..].find(close).unwrap() + a;
    s[a..b].to_string()
}
fn main() {
    let root = Path::new("/home/ubuntu/src/zopro-perf-work/vnni13-screen175");
    let strings = fs::read_to_string(root.join("sources/sharedStrings.xml")).unwrap();
    let strings: Vec<_> = strings.split("<si>").skip(1).map(|s| {
        let s = s.split("</si>").next().unwrap();
        assert_eq!(s.matches("<t>").count(), 1);
        text(s, "<t>", "</t>")
    }).collect();
    let xml = fs::read_to_string(root.join("sources/sheet2.xml")).unwrap();
    let mut selected = Vec::new();
    let mut headers = BTreeMap::new();
    for row in xml.split("<row ").skip(1) {
        let row = row.split("</row>").next().unwrap();
        let mut fields = BTreeMap::new();
        for cell in row.split("<c ").skip(1) {
            let cell = cell.split("</c>").next().unwrap();
            if !cell.contains("<v>") { continue; }
            let reference = text(cell, "r=\"", "\"");
            let col: String = reference.chars().take_while(|c| c.is_ascii_alphabetic()).collect();
            let mut value = text(cell, "<v>", "</v>");
            if cell.split('>').next().unwrap().contains("t=\"s\"") {
                value = strings[value.parse::<usize>().unwrap()].clone();
            }
            fields.insert(col, value);
        }
        if headers.is_empty() { headers = fields; continue; }
        let name = fields.get("A").map(String::as_str).unwrap_or("");
        if ["vpdpwssd", "vpmadd52luq", "vpmadd52huq"].contains(&name) {
            let mut result = BTreeMap::new();
            for (col, value) in &fields { result.insert(headers[col].clone(), value.clone()); }
            result.insert("worksheet_row".into(), text(row, "r=\"", "\""));
            selected.push(result);
        }
    }
    assert!(!selected.is_empty());
    let output = json!({"document":"AMD 58455pub revision 1.00 (2024-08-15)","sheet":"xl/worksheets/sheet2.xml","headers":headers,"rows":selected});
    fs::write(root.join("instruction-rows.json"), serde_json::to_vec_pretty(&output).unwrap()).unwrap();
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}
