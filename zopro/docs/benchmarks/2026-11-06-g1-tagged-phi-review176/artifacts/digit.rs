fn digit_at(s: &[u64; 4], span: Span) -> i16 {
    let Span { start, width } = span;
    if start == 0 {
        let value = s[0] & ((1u64 << width) - 1);
        return (value as i64 - ((value >> (width - 1)) << width) as i64) as i16;
    }
    let offset = start - 1;
    let (word, shift) = (offset / 64, offset % 64);
    let mut value = if word < 4 { s[word] >> shift } else { 0 };
    if shift + width + 1 > 64 && word < 3 {
        value |= s[word + 1] << (64 - shift);
    }
    value &= (1u64 << (width + 1)) - 1;
    (((value + 1) >> 1) as i64 - ((value >> width) << width) as i64) as i16
}