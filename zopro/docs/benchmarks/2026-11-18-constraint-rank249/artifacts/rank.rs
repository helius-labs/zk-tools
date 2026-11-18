#[derive(Default)]
struct Basis {
    pivots: BTreeMap<usize, BTreeMap<usize,Fr>>,
}

impl Basis {
    fn insert(&mut self, form: &Form) -> bool {
        // Column zero represents the constant function one.
        let mut row: BTreeMap<_,_> = form.terms.iter().map(|(&atom,&c)|(atom+1,c)).collect();
        if form.constant != Fr::ZERO {row.insert(0,form.constant);}
        while let Some((&column,&factor))=row.last_key_value() {
            if let Some(pivot)=self.pivots.get(&column) {
                for (&i,&coeff) in pivot {
                    let v=row.entry(i).or_insert(Fr::ZERO);
                    *v-=factor*coeff;
                    if *v==Fr::ZERO {row.remove(&i);}
                }
            } else {
                if factor!=Fr::ONE {
                    let inv=factor.inverse().unwrap();
                    for v in row.values_mut(){*v*=inv;}
                }
                self.pivots.insert(column,row);
                return true;
            }
        }
        false
    }
}

fn query_rank(q:&[usize],state:&[Option<Form>],relations:&[Form])->Value {
    let mut regular=Basis::default();
    let mut constrained=Basis::default();
    for f in relations {constrained.insert(f);}
    let relations_rank=constrained.pivots.len();
    assert!(!constrained.pivots.contains_key(&0),"relations imply a nonzero constant is zero");
    for &i in q {
        let form=state[i].as_ref().unwrap();
        regular.insert(form);
        constrained.insert(form);
    }
    let dimension=constrained.pivots.len()-relations_rank;
    assert!(dimension<=regular.pivots.len());
    json!({"rows":q.len(),"unconditional_rank":regular.pivots.len(),"constraint_only_rows":relations.len(),"relation_rank":relations_rank,"quotient_rank":dimension,"conditional_rows_removed":q.len()-dimension})
}

#[cfg(test)]
mod rank_tests {
    use super::*;
    fn dense_rank(mut a:Vec<Vec<Fr>>) ->usize {
        let cols=a[0].len();
        let mut rank=0;
        for col in 0..cols {
            let Some(p)=(rank..a.len()).find(|&r|a[r][col]!=Fr::ZERO) else {continue};
            a.swap(rank,p);
            let inv=a[rank][col].inverse().unwrap();
            let normalized:Vec<_>=a[rank].iter().map(|&v|v*inv).collect();
            for row in &mut a[rank+1..] {
                let x=row[col];
                for (entry,&pivot) in row.iter_mut().zip(&normalized){*entry-=x*pivot;}
            }
            rank+=1;
        }
        rank
    }
    #[test] fn sparse_matches_dense_quotient_in_both_orders() {
        for seed in 0..16_u64 {
            let raw:Vec<Vec<_>>=(0..11).map(|i|(0..6).map(|j|
                Fr::from(if j==0&&i<4{0}else{((seed+i*i+3*j)*(i+2*j+1))%5})
            ).collect()).collect();
            let forms:Vec<_>=raw.iter().map(|r|Form{constant:r[0],terms:r[1..].iter().enumerate().filter_map(|(j,&v)|(v!=Fr::ZERO).then_some((j,v))).collect()}).collect();
            let state=forms.iter().cloned().map(Some).collect::<Vec<_>>();
            let qs=(4..11).collect::<Vec<_>>();
            let expected=dense_rank(raw.clone())-dense_rank(raw[..4].to_vec());
            for order in [qs.clone(),qs.iter().rev().copied().collect()] {
                let found=query_rank(&order,&state,&forms[..4]);
                assert_eq!(found["quotient_rank"],expected);
                assert_eq!(found["unconditional_rank"],dense_rank(raw[4..].to_vec()));
            }
        }
    }
}
