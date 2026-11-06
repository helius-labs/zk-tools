struct NativeBuckets<C:CurveConfig<Base=Fq2>> {
    coords:[zopro_bn254::native_bucket_store::Store;4], len:usize,
    marker:core::marker::PhantomData<C>,
}
impl<C:CurveConfig<Base=Fq2>> NativeBuckets<C> {
    fn new(points:&[Affine<C>])->Self {
        let mut s=Self{coords:core::array::from_fn(|_|zopro_bn254::native_bucket_store::Store::new(points.len())),len:points.len(),marker:core::marker::PhantomData};
        for (i,&p) in points.iter().enumerate(){s.set(i,p);}s
    }
    #[inline(always)] fn point(&self,i:usize)->Affine<C>{Affine::new(Fq2::new(self.coords[0].get(i),self.coords[1].get(i)),Fq2::new(self.coords[2].get(i),self.coords[3].get(i)))}
    #[inline(always)] fn set(&mut self,i:usize,p:Affine<C>){for(j,v)in[p.x.c0,p.x.c1,p.y.c0,p.y.c1].into_iter().enumerate(){self.coords[j].set(i,v);}}
    #[inline(always)] fn x(&self,ids:[u32;8])->[FqBatch;2]{core::array::from_fn(|j|FqBatch::native_gather(&self.coords[j],ids))}
    #[inline(always)] fn y(&self,ids:[u32;8])->[FqBatch;2]{core::array::from_fn(|j|FqBatch::native_gather(&self.coords[j+2],ids))}
    #[inline(always)] fn put(&mut self,ids:[u32;8],x:[FqBatch;2],y:[FqBatch;2]){for(j,v)in x.into_iter().chain(y).enumerate(){v.native_scatter(&mut self.coords[j],ids);}}
    fn points(&self)->Vec<Affine<C>>{(0..self.len).map(|i|self.point(i)).collect()}
}
