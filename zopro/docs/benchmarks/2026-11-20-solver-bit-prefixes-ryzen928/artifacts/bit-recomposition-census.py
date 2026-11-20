from pathlib import Path
import hashlib
import json
import sys
from collections import Counter
from struct import unpack_from

P=21888242871839275222246405745257275088548364400416034343698204186575808495617
R=pow(1<<256,-1,P)

def scan(image):
    data=image.read_bytes()
    assert data[:16]==b'zopro-euler-v2\0\0'
    head,=unpack_from('<Q',data,16)
    _,_,_,a,b,k,z,b2=unpack_from('<8I',data,44)
    offset=(24+head+63)&~63
    ns,np=unpack_from('<2Q',data,offset)
    offset=(offset+128+64*(a+b+k+ns+z+1+np)+128*b2+8*ns+63)&~63
    size,=unpack_from('<Q',data,offset)
    offset+=8
    assert len(data)-offset==size and data[offset:offset+8]==b'ZSP1\x02\0\0\0'
    _,_,nw,ni,_,_,na,nb,ncoeff,nt,nsums,nslots,no=unpack_from('<13I',data,offset)
    offset+=52+4*(na+nb)
    coeff=[int.from_bytes(data[offset+32*i:offset+32*(i+1)],'little')*R%P for i in range(ncoeff)]
    assert coeff[:5]==[0,1,2,P-1,P-2]
    offset+=32*ncoeff
    terms=[unpack_from('<2I',data,offset+8*i) for i in range(nt)]
    offset+=8*nt
    sums=[unpack_from('<3I',data,offset+12*i) for i in range(nsums)]
    offset+=12*nsums
    bits={}
    coverage=Counter()
    patterns=Counter()
    run_lengths=Counter()
    whole_lengths=Counter()
    whole_runs=[]
    spans=[]
    chained=0

    def tally(record):
        coeff_id,wire=record
        value=coeff[coeff_id]
        coverage['terms']+=1
        if wire in bits:
            coverage['bit_terms']+=1
            if coeff_id>4:
                coverage['bit_mul_terms']+=1
            if value.bit_count()==1 and 0<value<P:
                coverage['pow2_bit_terms']+=1
            if (P-value).bit_count()==1:
                coverage['negative_pow2_bit_terms']+=1
        if wire and coeff_id>4:
            coverage['general_mul_terms']+=1

    for n in range(no):
        tag,=unpack_from('<I',data,offset)
        if tag==1:
            kind,is_,ie,os,oe=unpack_from('<5I',data,offset+4)
            for term in terms[is_:ie]:
                tally(term)
            if kind==1:
                bits.update((wire,(n,wire-os)) for wire in range(os,oe))
            offset+=24
            continue
        assert tag==0
        row,ss,se,slot=unpack_from('<4I',data,offset+4)
        expr=[unpack_from('<3I',data,offset+20+12*j) for j in range(3)]
        assign,=unpack_from('<I',data,offset+56)
        if assign:
            side,wire,coef=unpack_from('<3I',data,offset+60)
            patterns[(side,tuple(x[0] for x in expr),bool(se-ss),coef<=4)]+=1
            offset+=104
        else:
            patterns[(3,tuple(x[0] for x in expr),bool(se-ss),False)]+=1
            offset+=60
        for kind,start,end in expr:
            if kind==3:
                tally(terms[start])
            elif kind==4:
                for term in terms[start:end]:
                    tally(term)
        if se-ss >= 8:
            words=sums[ss:se]
            values=[coeff[co] for _,co,_ in words]
            positive=all(x.bit_count()==1 and x.bit_length()<=253 for x in values)
            negative=all((P-x).bit_count()==1 and (P-x).bit_length()<=253 for x in values)
            positions=[(x if positive else P-x).bit_length()-1 for x in values]
            if ((positive or negative) and len(set(positions))==len(words)
                and all(wire in bits for _,_,wire in words)
                and all((word[0]&0x7fffffff)==0x7ffffffe for word in words[1:])
                and all(word[0]<0x80000000 for word in words[:-1])
                and words[-1][0]>>31):
                whole_lengths[len(words)]+=1
                delta=[b-a for a,b in zip(positions,positions[1:])]
                whole_runs.append(dict(row=row,length=len(words),negative=negative,first=min(positions),
                                       last=max(positions),ordered=all(d==1 for d in delta) or all(d==-1 for d in delta)))
        weighted=[]
        for prev,c,wire in sums[ss:se]:
            field=coeff[c]
            if wire in bits and field.bit_count()==1 and field.bit_length()<=253:
                weighted.append((field.bit_length(),prev))
            elif wire in bits and (P-field).bit_count()==1 and (P-field).bit_length()<=253:
                weighted.append((-(P-field).bit_length(),prev))
            else:
                weighted.append((0,prev))
        left=0
        while left+8<=len(weighted):
            first=weighted[left][0]
            second=weighted[left+1][0]
            diff=second-first
            if abs(diff)!=1 or first*second<=0:
                left+=1
                continue
            right=left+1
            for j in range(left+1,len(weighted)):
                signpower,prior=weighted[j]
                if (weighted[j-1][1]>>31 or prior&0x7fffffff!=0x7ffffffe
                    or signpower*first<=0 or signpower!=first+diff*(j-left)):
                    break
                right=j+1
            if right-left<8:
                left+=1
                continue
            spans.append(dict(row=row,offset=left,length=right-left,negative=first<0))
            left=right
        for j,(previous,co,wire) in enumerate(sums[ss:se]):
            tally((co,wire))
            coefficient=coeff[co]
            power=coefficient.bit_length()-1 if coefficient.bit_count()==1 else -1
            eligible=(wire in bits and 0<=power<=252)
            previous_tag=previous&0x7fffffff
            if chained and (not eligible or previous_tag!=0x7ffffffe):
                run_lengths[chained]+=1
                chained=0
            if eligible:
                chained+=1
            if previous>>31 and chained:
                run_lengths[chained]+=1
                chained=0
        if chained:
            run_lengths[chained]+=1
            chained=0
    assert offset==len(data)
    return dict(theoretical=coverage,run_lengths=run_lengths,whole_lengths=whole_lengths,whole_runs=whole_runs,spans=spans,patterns=[
        dict(side=s,types=t,sums=c,simple=a,total=n) for (s,t,c,a),n in patterns.most_common(16)],
        sha256=hashlib.sha256(data).hexdigest(),hints_bit_outputs=len(bits))

if __name__=='__main__':
    if not sys.argv[1:]:
        raise SystemExit('usage: bit-recomposition-census.py IMAGE ...')
    images=[Path(arg) for arg in sys.argv[1:]]
    results={p.stem:scan(p) for p in images}
    print(json.dumps(results,indent=2))
