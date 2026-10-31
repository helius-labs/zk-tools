// Semantic nodes are analysis-only; they are never a retained prover plan.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FormKey([u64;4],Vec<(usize,[u64;4])>);
impl Form {
    fn key(&self)->FormKey{FormKey(self.constant.to_canonical(),self.terms.iter().map(|(&i,c)|(i,c.to_canonical())).collect())}
    fn monic(&self)->(Fr,Self){
        if let Some((_,&c))=self.terms.first_key_value(){(c,self.scale(c.inverse().unwrap()))}
        else if self.constant!=Fr::ZERO{(self.constant,Self::constant(Fr::ONE))}
        else{(Fr::ZERO,self.clone())}
    }
}
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord)]
enum NodeKey{Mul(FormKey,FormKey),Inv(FormKey),Bit(FormKey,usize),Div(FormKey,FormKey,FormKey)}
#[derive(Clone,Debug)]
enum Node{Input(usize),Mul(Form,Form),Inv(Form),Bit(Form,usize),Div(Form,Form,Form)}
#[derive(Default,Debug)]
struct Stats{products:usize,reused_products:usize,inverses:usize,reused_inverses:usize,bits:usize,reused_bits:usize,divisions:usize,reused_divisions:usize,bool_square:usize,bool_complement:usize,bool_inverse:usize,bool_bits:usize,involution:usize,high_zero_bits:usize,wire_exact:usize,wire_proportional:usize}
struct Engine{
    next:usize,nodes:BTreeMap<usize,Node>,intern:BTreeMap<NodeKey,usize>,boolean:BTreeSet<usize>,
    exact_wires:BTreeMap<FormKey,usize>,proportional_wires:BTreeMap<FormKey,usize>,stats:Stats,
}
impl Engine{
    fn new(nw:usize)->Self{Self{next:nw,nodes:BTreeMap::new(),intern:BTreeMap::new(),boolean:BTreeSet::new(),exact_wires:BTreeMap::new(),proportional_wires:BTreeMap::new(),stats:Stats::default()}}
    fn input(&mut self,w:usize)->Form{assert!(w<self.next);assert!(self.nodes.insert(w,Node::Input(w)).is_none());Form::atom(w)}
    fn intern(&mut self,k:NodeKey,n:Node)->(Form,bool){
        if let Some(&i)=self.intern.get(&k){return(Form::atom(i),true);}
        let i=self.next;self.next+=1;assert!(self.nodes.insert(i,n).is_none());self.intern.insert(k,i);(Form::atom(i),false)
    }
    // The monic representatives b and b-1 have values{0,1} and{0,-1}.
    fn bit_shape(&self,a:&Form)->Option<(usize,bool)>{
        if a.terms.len()!=1{return None;}let(&i,&c)=a.terms.first_key_value().unwrap();
        if c!=Fr::ONE||!self.boolean.contains(&i){return None;}
        if a.constant==Fr::ZERO{Some((i,false))}else if a.constant== -Fr::ONE{Some((i,true))}else{None}
    }
    fn mul(&mut self,a:&Form,b:&Form)->Form{
        self.stats.products+=1;
        if let Some(c)=a.value(){return b.scale(c);}if let Some(c)=b.value(){return a.scale(c);}
        let(ca,mut a)=a.monic();let(cb,mut b)=b.monic();let c=ca*cb;
        if let(Some((ia,sa)),Some((ib,sb)))=(self.bit_shape(&a),self.bit_shape(&b)){
            if ia==ib{if sa!=sb{self.stats.bool_complement+=1;return Form::constant(Fr::ZERO);}
                self.stats.bool_square+=1;return a.scale(if sa{-c}else{c});}
        }
        if a.key()>b.key(){std::mem::swap(&mut a,&mut b);}
        let key=NodeKey::Mul(a.key(),b.key());let(f,reused)=self.intern(key,Node::Mul(a,b));self.stats.reused_products+=usize::from(reused);f.scale(c)
    }
    fn inv0(&mut self,a:&Form)->Form{
        self.stats.inverses+=1;
        if let Some(c)=a.value(){return Form::constant(c.inverse().unwrap_or(Fr::ZERO));}
        let(c,a)=a.monic();let inverse=c.inverse().unwrap();
        if self.bit_shape(&a).is_some(){self.stats.bool_inverse+=1;return a.scale(inverse);}
        if a.constant==Fr::ZERO&&a.terms.len()==1{
            let(&i,&v)=a.terms.first_key_value().unwrap();assert_eq!(v,Fr::ONE);
            if let Some(Node::Inv(original))=self.nodes.get(&i){self.stats.involution+=1;return original.scale(inverse);}
        }
        let key=NodeKey::Inv(a.key());let(f,reused)=self.intern(key,Node::Inv(a));self.stats.reused_inverses+=usize::from(reused);f.scale(inverse)
    }
    fn bit(&mut self,a:&Form,index:usize)->Form{
        assert!(index<256);self.stats.bits+=1;
        if index>=Fr::MODULUS_BITS as usize{self.stats.high_zero_bits+=1;return Form::constant(Fr::ZERO);}
        if let Some(c)=a.value(){let w=c.to_canonical();return Form::constant(if w[index/64]>>(index%64)&1!=0{Fr::ONE}else{Fr::ZERO});}
        let(c,normal)=a.monic();if let Some((id,complement))=self.bit_shape(&normal){
            self.stats.bool_bits+=1;let c=if complement{-c}else{c};let words=c.to_canonical();
            return if words[index/64]>>(index%64)&1==0{Form::constant(Fr::ZERO)}else if complement{Form::constant(Fr::ONE).minus(&Form::atom(id))}else{Form::atom(id)};
        }
        let key=NodeKey::Bit(a.key(),index);let(f,reused)=self.intern(key,Node::Bit(a.clone(),index));self.stats.reused_bits+=usize::from(reused);let id=*f.terms.first_key_value().unwrap().0;self.boolean.insert(id);f
    }
    fn assign(&mut self,side:usize,v:&[Form;3],inverse:Fr)->Form{
        if side==2{return self.mul(&v[0],&v[1]).minus(&v[2]).scale(inverse);}
        if let Some(f)=assignment(side,v,inverse){return f;}
        self.stats.divisions+=1;
        let(c,other)=v[1-side].monic();assert_ne!(c,Fr::ZERO);let output=v[2].scale(c.inverse().unwrap());let own=v[side].clone();
        let key=NodeKey::Div(other.key(),output.key(),own.key());let(f,reused)=self.intern(key,Node::Div(other,output,own));self.stats.reused_divisions+=usize::from(reused);f.scale(inverse)
    }
    fn note_wire(&mut self,w:usize,a:&Form){
        self.stats.wire_exact+=usize::from(self.exact_wires.insert(a.key(),w).is_some());
        let(c,normal)=a.monic();if c!=Fr::ZERO{self.stats.wire_proportional+=usize::from(self.proportional_wires.insert(normal.key(),w).is_some());}
    }
    fn summary(&self)->Value{json!({"semantic_nodes":self.nodes.len(),"interned_nonlinear_nodes":self.intern.len(),"Boolean_atoms":self.boolean.len(),"product_requests":self.stats.products,"reused_products":self.stats.reused_products,"inverse_hint_requests":self.stats.inverses,"reused_inverse_hints":self.stats.reused_inverses,"bit_hint_output_requests":self.stats.bits,"reused_bit_outputs":self.stats.reused_bits,"guarded_division_requests":self.stats.divisions,"reused_guarded_divisions":self.stats.reused_divisions,"Boolean_square_rules":self.stats.bool_square,"Boolean_complement_zero_rules":self.stats.bool_complement,"Boolean_inverse_rules":self.stats.bool_inverse,"Boolean_bit_rules":self.stats.bool_bits,"inverse_involution_rules":self.stats.involution,"constant_high_bit_rules":self.stats.high_zero_bits,"wire_exact_alias_writes":self.stats.wire_exact,"wire_proportional_alias_writes":self.stats.wire_proportional})}
    #[cfg(test)]fn evaluate(&self,f:&Form,inputs:&[Fr])->Result<Fr,String>{
        let mut values=BTreeMap::new();let value=|f:&Form,v:&BTreeMap<usize,Fr>|->Fr{f.terms.iter().fold(f.constant,|a,(i,c)|a+*c*v[i])};
        for(&id,n)in&self.nodes{let out=match n{
            Node::Input(w)=>*inputs.get(*w).ok_or("missing input")?,
            Node::Mul(a,b)=>value(a,&values)*value(b,&values),
            Node::Inv(a)=>value(a,&values).inverse().unwrap_or(Fr::ZERO),
            Node::Bit(a,i)=>{let x=value(a,&values).to_canonical();if x[i/64]>>(i%64)&1==1{Fr::ONE}else{Fr::ZERO}},
            Node::Div(other,output,own)=>{let o=value(other,&values);let r=value(output,&values);if o==Fr::ZERO{if r!=Fr::ZERO{return Err("unsatisfied zero-divisor assignment".into());}Fr::ZERO}else{r*o.inverse().unwrap()-value(own,&values)}}
        };values.insert(id,out);}Ok(value(f,&values))
    }
}
fn query_classes(q:&[usize],state:&[Option<Form>])->Value{
    let mut exact=BTreeSet::new();let mut proportional=BTreeMap::<FormKey,usize>::new();let(mut zeros,mut constants,mut terms)=(0,0,0);
    for&w in q{let f=state[w].as_ref().unwrap();terms+=f.terms.len();if f.value()==Some(Fr::ZERO){zeros+=1;continue;}constants+=usize::from(f.value().is_some());exact.insert(f.key());let(_,m)=f.monic();*proportional.entry(m.key()).or_default()+=1;}
    let merged=proportional.values().filter(|&&n|n>1).count();let max=proportional.values().copied().max().unwrap_or(0);
    json!({"points":q.len(),"zero_rows":zeros,"nonzero_constant_rows":constants,"exact_nonzero_classes":exact.len(),"proportional_nonzero_classes":proportional.len(),"potential_terms_removed_exact_or_zero":q.len()-exact.len(),"potential_terms_removed_proportional_or_zero":q.len()-proportional.len(),"merged_proportional_classes":merged,"maximum_class_size":max,"affine_atom_terms":terms})
}
