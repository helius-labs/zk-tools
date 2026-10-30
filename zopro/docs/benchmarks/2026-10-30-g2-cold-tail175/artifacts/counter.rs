fn event_line(line:&str)->Option<(String,Value)>{
    let line=line.trim();if line.is_empty()||line.starts_with('#'){return None;}let f=line.split(';').collect::<Vec<_>>();if f.len()<5{return Some(("unparsed".into(),json!({"raw":line,"qualified":false})));}
    let number=|i:usize|f[i].trim().parse::<f64>().ok().filter(|v|v.is_finite()&&*v>=0.);
    let count=number(0);let runtime=number(3);let coverage=number(4);let qualified=count.is_some()&&runtime.is_some_and(|v|v>0.)&&coverage.is_some_and(|v|v>=99.9);
    Some((f[2].trim().into(),json!({"count":count,"runtime_ns":runtime,"running_percent":coverage,"qualified":qualified,"raw":line})))
}
