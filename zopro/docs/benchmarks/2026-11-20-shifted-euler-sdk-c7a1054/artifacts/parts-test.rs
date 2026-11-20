use super::*;

#[cfg(test)]
#[test]
fn shifted_12x2_independent_parts_match_standard() {
    use std::sync::Arc;
    use zopro_bn254::Fr;

    let source = std::fs::read(std::env::var_os("ZOPRO_TRIAL_STANDARD_IMAGE").unwrap()).unwrap();
    let head_len = u64::from_le_bytes(source[16..24].try_into().unwrap()) as usize;
    let (artifact, _) = Artifact::head(&source[24..24+head_len]).unwrap();
    let [a, b, k, _, _] = artifact.points.map(|v| v as usize);
    let standard = Prover::load_image(Arc::new(source)).unwrap();
    let shifted = Prover::load_image(Arc::new(
        std::fs::read(std::env::var_os("ZOPRO_TRIAL_SHIFTED_IMAGE").unwrap()).unwrap()
    )).unwrap();

    let words = |len: usize, channel: usize, seed: usize| {
        let mut bytes = Vec::with_capacity(len * 32);
        let mut state = 0xa9ce6845e629924du64 ^ ((channel as u64) << 21) ^ seed as u64;
        for i in 0..len {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let f = if seed > 0 && (channel >= 3 || (i + channel) % 67 < seed) {
                Fr::from(state)
            } else {
                Fr::ZERO
            };
            bytes.extend(f.0.into_iter().flat_map(u64::to_le_bytes));
        }
        bytes
    };
    for (seed, n) in [0, 1, 8195].into_iter().enumerate() {
        let [sa,sb,sk,left,right,out] = [a,b,k,n,n,n].into_iter().enumerate()
            .map(|(i,len)| words(len,i,seed)).collect::<Vec<_>>().try_into().unwrap();
        let ordinary = standard.key.parts(&sa,&sb,&sk,&left,&right,&out)
            .unwrap_or_else(|_| panic!("standard independent parts failed at seed {seed}"));
        let centered = shifted.key.parts(&sa,&sb,&sk,&left,&right,&out)
            .unwrap_or_else(|_| panic!("shifted independent parts failed at seed {seed}"));
        assert_eq!(ordinary,centered,"independent parts differ at seed {seed}");
    }
}
