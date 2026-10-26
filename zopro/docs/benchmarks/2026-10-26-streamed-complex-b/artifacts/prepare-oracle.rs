use std::{fs, path::Path};
fn main() {
    let root = Path::new("/tmp/zopro-streamed-complex-b");
    let mut text = fs::read_to_string("/tmp/zopro-phased-complex-products/probe-tests.rs").unwrap();
    text = text
        .replace("complex_product_phased", "complex_product_streamed")
        .replace("EXACT_PHASED", "EXACT_STREAMED")
        .replace("ARK_PHASED", "ARK_STREAMED");
    let start = text.find("    fn model(").unwrap();
    let end = text[start..].find("    macro_rules! lane_tests").unwrap() + start;
    text.replace_range(
        start..end,
        &fs::read_to_string(root.join("row-model.rs.txt")).unwrap(),
    );
    let old="let got=Vector::complex_product_streamed(&v[0],&v[1],&v[2],&v[3]);let gd=got.each_ref().map(raw);";
    assert_eq!(text.matches(old).count(), 1);
    text=text.replace(old,&(old.to_owned()+"let previous=Vector::complex_product_shared(&v[0],&v[1],&v[2],&v[3]);assert_eq!(gd,previous.each_ref().map(raw));"));
    let start = text
        .find("                            let inputs=[digits(&a0)")
        .unwrap();
    let end = text[start..]
        .find("                                let exact=exact_redc(")
        .unwrap()
        + start;
    text.replace_range(start..end,"                            let inputs=[digits(&a0),digits(&a1),digits(&b0),digits(&b1)];let modeled=model(inputs,digits(&p),C::INV,MIXED,&mut max);\n                            for (c,t,bias) in [(0,real,true),(1,BigInt::from(imag),false)] {\n");
    let old = "assert_eq!(model(coeff,digits(&p),C::INV,MIXED,bias,&mut max),exact);";
    assert_eq!(text.matches(old).count(), 1);
    text = text.replace(old, "assert_eq!(modeled[c],exact);");
    fs::write(root.join("probe-tests.rs"), text).unwrap();
}
