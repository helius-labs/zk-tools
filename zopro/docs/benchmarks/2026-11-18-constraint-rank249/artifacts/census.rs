mod model;
use std::{env,fs};
fn main(){
    let p=env::args().nth(1).unwrap();
    let v=model::census(&fs::read(p).unwrap()).unwrap();
    println!("{}",serde_json::json!({"rank":v["rank"],"proportional":v["semantic_query_classes"],"bytes":v["bytes"],"constraints":v["constraints"]}));
}
