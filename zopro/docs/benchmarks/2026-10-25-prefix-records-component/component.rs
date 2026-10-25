use std::{fs,path::Path};
fn main(){
    let output=std::env::args().nth(1).unwrap();let out=Path::new(&output);assert!(!out.exists());fs::create_dir_all(out).unwrap();
    let mut mask:libc::cpu_set_t=unsafe{std::mem::zeroed()};unsafe{libc::CPU_ZERO(&mut mask);libc::CPU_SET(0,&mut mask);}
    assert_eq!(unsafe{libc::sched_setaffinity(0,std::mem::size_of_val(&mask),&mask)},0);
    assert_eq!(unsafe{libc::sched_getcpu()},0);
    let cases=[(1u8,200usize),(1,352),(2,200)];let mut rows=Vec::new();
    for block in 0..31usize{
        let mut order=cases.to_vec();order.rotate_left(block%3);if block%2==1{order.reverse();}
        for (curve,batch)in order{
            let order=if block%2==0{[0,1]}else{[1,0]};let mut expected=None;
            for mode in order{
                let(ns,bytes,checksum)=zopro_core::prefix_records_component(curve,mode,batch,256);
                if let Some(previous)=&expected{assert_eq!(previous,&checksum);}else{expected=Some(checksum);}
                rows.push(serde_json::json!({"block":block,"curve":curve,"batch":batch,"mode":mode,"iterations":256,"ns":ns,"scratch_capacity_bytes":bytes}));
            }
        }
    }
    assert_eq!(unsafe{libc::sched_getcpu()},0);
    fs::write(out.join("rows.json"),serde_json::to_vec_pretty(&rows).unwrap()).unwrap();
    println!("PREFIX_COMPONENT_COMPLETE records={} cpu=0",rows.len());
}
