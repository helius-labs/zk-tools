fn booth_digit(words:&[u64;4],w:usize,width:usize)->i16 {
    if w==0 {let value=words[0]&((1u64<<width)-1);return (value as i64-((value>>(width-1))<<width)as i64)as i16;}
    let offset=w*width-1;let(word,shift)=(offset/64,offset%64);
    let mut value=if word<4 {words[word]>>shift}else{0};
    if shift+width+1>64&&word<3 {value|=words[word+1]<<(64-shift);}
    value&=(1u64<<(width+1))-1;
    (((value+1)>>1)as i64-((value>>width)<<width)as i64)as i16
}

