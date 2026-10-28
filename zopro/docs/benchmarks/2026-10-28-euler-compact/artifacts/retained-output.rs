#[inline]
fn euler_combine_products(a: &mut [Fr], b: &[Fr], values: &[PackedPair]) {
    if cfg!(zopro_euler_pointwise8) {
        a.par_chunks_mut(8).zip(values.par_chunks(2)).enumerate().for_each(|(i,(out,values))| {
            let (da0,db0) = values[0].load().halves();
            let (da1,db1) = values[1].load().halves();
            let da = Pair::from_halves(&da0,&da1);
            let db = Pair::from_halves(&db0,&db1);
            let aa = Pair::new(&core::array::from_fn(|j| out.get(j).copied().unwrap_or(Fr::ZERO)));
            let bb = Pair::new(&core::array::from_fn(|j| b.get(8*i+j).copied().unwrap_or(Fr::ZERO)));
            let product = da.sum2(&bb,&aa,&db).get();
            out.copy_from_slice(&product[..out.len()]);
        });
    } else {
    a.par_chunks_mut(4).zip(values.par_iter()).enumerate().for_each(|(i,(out,value))| {
        let (da,db) = Quad::unpair(&value.load());
        let aa = Quad::new(&core::array::from_fn(|j| out.get(j).copied().unwrap_or(Fr::ZERO)));
        let bb = Quad::new(&core::array::from_fn(|j| b.get(4*i+j).copied().unwrap_or(Fr::ZERO)));
        let product = da.mul(&bb).add(&aa.mul(&db)).get();
        out.copy_from_slice(&product[..out.len()]);
    });
    }
}
