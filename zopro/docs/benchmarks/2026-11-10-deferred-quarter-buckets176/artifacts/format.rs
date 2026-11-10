    /// Keep original canonical H words until a vector/scalar consumer needs them.
    #[inline(always)]
    pub fn from_original(y:Fq)->Self {
        debug_assert!(crate::portable::less(y.0,FqConfig::P));
        let mut w=y.0;w[3]|=1u64<<63;Self(w)
    }
    #[inline(always)]
    fn original(&self)->bool{self.0[3]>>63!=0}
    #[inline(always)]
    fn payload(self)->[u64;4]{let mut w=self.0;w[3]&=!(1u64<<63);w}
    #[inline(always)]
    pub fn is_zero(self)->bool {
        let w=self.payload();w==[0;4]||!self.original()&&w==FqConfig::P
    }
    pub fn decode(self)->Fq {
        let w=self.payload();
        if self.original(){Fq::from_montgomery(w)}
        else{Fq::from_montgomery(crate::portable::reduce_once(w,FqConfig::P)).double().double()}
    }
