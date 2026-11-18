mod model;
fn main() {let path=std::env::args().nth(1).unwrap();let v=model::census(&std::fs::read(path).unwrap()).unwrap();println!("{}",v);}
