fn classify(batches:&[Vec<u32>],start:usize,end:usize)->Value{
    let mut groups=0u64;let mut pairs=0u64;let mut tails=0u64;let mut classes=[0u64;4];let mut blocks=[0u64;9];let mut edges=0u64;let mut seen=std::collections::BTreeSet::new();
    for batch in batches {pairs+=batch.len()as u64;tails+=(batch.len()%8)as u64;for &i in batch{assert!((start..end).contains(&(i as usize)));assert!(seen.insert(i),"source repeated within component");}
        for g in batch.chunks_exact(8){groups+=1;let consecutive=g.windows(2).all(|a|a[0].checked_add(1)==Some(a[1]));let mut tile:[u32;8]=std::array::from_fn(|i|g[i]/8);tile.sort_unstable();let count=1+tile.windows(2).filter(|a|a[0]!=a[1]).count();blocks[count]+=1;let kind=if consecutive&&g[0]%8==0{0}else if consecutive{1}else if count==1{2}else{3};classes[kind]+=1;edges+=g.windows(2).filter(|a|a[0].checked_add(1)==Some(a[1])).count()as u64;}
    }
    assert_eq!(pairs,groups*8+tails);assert_eq!(classes.iter().sum::<u64>(),groups);assert_eq!(blocks.iter().sum::<u64>(),groups);
    json!({"batches":batches.len(),"pairs":pairs,"full_groups":groups,"scalar_tail_pairs":tails,"aligned_consecutive":classes[0],"unaligned_consecutive":classes[1],"single_block_permutation":classes[2],"general_gather":classes[3],"distinct_source_blocks_histogram":blocks,"consecutive_edges":edges,"possible_edges":7*groups,"unique_source_ordinals":seen.len()})
}
