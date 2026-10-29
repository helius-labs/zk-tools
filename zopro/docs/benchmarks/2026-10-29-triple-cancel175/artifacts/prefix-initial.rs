impl<C:FpConfig,const MIXED:bool> Ifma8<C,MIXED>{
    // -p^-1 modulo2^156, generated at compile time by exact bit lifting.
    const TRIPLE_INV:[u64;3]={
        let mut q=[0u64;3];
        let p=[C::P[0],C::P[1],C::P[2]];
        let mut bit=0;
        while bit<156 {
            let mut product=[0u64;3];let mut i=0;
            while i<3{let mut carry=0u128;let mut j=0;while j<3-i{let x=(p[i]as u128)*(q[j]as u128)+product[i+j]as u128+carry;product[i+j]=x as u64;carry=x>>64;j+=1;}i+=1;}
            let mut carry=1u128;let mut i=0;while i<3{let x=product[i]as u128+carry;product[i]=x as u64;carry=x>>64;i+=1;}
            if (product[bit/64]>>(bit%64))&1!=0{q[bit/64]|=1u64<<(bit%64);}
            bit+=1;
        }
        [q[0]&MASK,((q[0]>>52)|(q[1]<<12))&MASK,((q[1]>>40)|(q[2]<<24))&MASK]
    };
    #[inline(always)]pub fn mul_triple(&self,b:&Self)->Self{self.product_triple::<false>(b,self,b)}
    #[inline(always)]pub fn sum2_triple(&self,b:&Self,c:&Self,d:&Self)->Self{self.product_triple::<true>(b,c,d)}
    #[inline(always)]fn prefix_three<const FUSED:bool>(&self,b:&Self,c:&Self,d:&Self)->[__m512i;6]{unsafe{
        let zero=Self::splat(0);let mask=Self::splat(MASK);let p=split(C::P).map(Self::splat);let inv=Self::TRIPLE_INV.map(Self::splat);let mut t=[zero;8];
        macro_rules! input{($i:literal)=>{{macro_rules! term{($j:literal)=>{{t[$j+$i]=_mm512_madd52lo_epu64(t[$j+$i],self.0[$j],b.0[$i]);t[$j+$i+1]=_mm512_madd52hi_epu64(t[$j+$i+1],self.0[$j],b.0[$i]);if FUSED{t[$j+$i]=_mm512_madd52lo_epu64(t[$j+$i],c.0[$j],d.0[$i]);t[$j+$i+1]=_mm512_madd52hi_epu64(t[$j+$i+1],c.0[$j],d.0[$i]);}};}term!(0);term!(1);term!(2);term!(3);term!(4);}};}
        input!(0);input!(1);input!(2);
        let u0=_mm512_and_si512(t[0],mask);
        let v1=_mm512_add_epi64(t[1],_mm512_srli_epi64::<52>(t[0]));
        let u1=_mm512_and_si512(v1,mask);
        let u2=_mm512_add_epi64(t[2],_mm512_srli_epi64::<52>(v1));
        let q0=_mm512_madd52lo_epu64(zero,u0,inv[0]);
        let h0=_mm512_madd52hi_epu64(zero,u0,inv[0]);
        let lo1=_mm512_madd52lo_epu64(h0,u0,inv[1]);
        let q1raw=_mm512_madd52lo_epu64(lo1,u1,inv[0]);
        let q1=_mm512_and_si512(q1raw,mask);
        let hi1=_mm512_madd52hi_epu64(zero,u0,inv[1]);
        let hi2=_mm512_madd52hi_epu64(hi1,u1,inv[0]);
        let lo2=_mm512_madd52lo_epu64(hi2,u0,inv[2]);
        let lo3=_mm512_madd52lo_epu64(lo2,u1,inv[1]);
        let q2raw=_mm512_madd52lo_epu64(lo3,u2,inv[0]);
        let q2=_mm512_and_si512(_mm512_add_epi64(q2raw,_mm512_srli_epi64::<52>(q1raw)),mask);
        macro_rules! cancel{($off:literal,$q:expr)=>{{let q=$q;let carry=_mm512_srli_epi64::<52>(_mm512_add_epi64(t[$off],mask));let low=_mm512_madd52hi_epu64(carry,q,p[0]);macro_rules! limb{($j:literal)=>{{t[$off+$j]=_mm512_madd52lo_epu64(t[$off+$j],q,p[$j]);t[$off+$j+1]=_mm512_madd52hi_epu64(t[$off+$j+1],q,p[$j]);}};}limb!(1);limb!(2);limb!(3);limb!(4);t[$off+1]=_mm512_add_epi64(t[$off+1],low);}};}
        cancel!(0,q0);cancel!(1,q1);cancel!(2,q2);
        [t[3],t[4],t[5],t[6],t[7],zero]
    }}
