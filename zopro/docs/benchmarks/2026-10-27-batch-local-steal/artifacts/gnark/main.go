package main

import (
 "bytes"
 "crypto/sha256"
 "encoding/binary"
 "encoding/hex"
 "fmt"
 "os"
 "github.com/consensys/gnark/backend/groth16"
 "github.com/consensys/gnark/backend/witness"
 "github.com/consensys/gnark-crypto/ecc"
)

func must(err error){if err!=nil{panic(err)}}
func bytesAt(data []byte,at *uint64,n uint64)[]byte{if *at>uint64(len(data))||n>uint64(len(data))-*at{panic("truncated input")};out:=data[*at:*at+n];*at+=n;return out}
func length(data []byte,at *uint64,wide bool)uint64{if wide{return binary.LittleEndian.Uint64(bytesAt(data,at,8))};return uint64(binary.LittleEndian.Uint32(bytesAt(data,at,4)))}
func verifyingKey(image []byte)[]byte{
 if len(image)<24{panic("short image")}
 magic:=string(image[:16]);wide:=false
 switch magic{case "zopro-euler-v2\x00\x00","zopro-basis-v2\x00\x00":wide=true;case "zopro-euler-v1\x00\x00","zopro-basis-v1\x00\x00":default:panic("unexpected image magic")}
 at:=uint64(16);n:=length(image,&at,wide);head:=bytesAt(image,&at,n)
 if len(head)<564||string(head[:16])!="zopro-key\x00\x00\x00\x00\x00\x00\x00"{panic("invalid artifact header")}
 version:=binary.LittleEndian.Uint32(head[16:20]);if version!=2&&version!=3{panic("artifact version")};at=564
 for i:=0;i<3;i++{n:=length(head,&at,version==3);blob:=bytesAt(head,&at,n);if i==2{return blob}}
 panic("missing verifying key")
}
func main(){
 if len(os.Args)!=6{panic("usage: verify IMAGE EXPECTED_IMAGE_SHA256 PROOF PUBLIC EXPECTED_PUBLIC_SHA256")}
 image,err:=os.ReadFile(os.Args[1]);must(err);imageHash:=sha256.Sum256(image);if hex.EncodeToString(imageHash[:])!=os.Args[2]{panic("image hash mismatch")}
 vkBytes:=verifyingKey(image);vk:=groth16.NewVerifyingKey(ecc.BN254);reader:=bytes.NewReader(vkBytes);_,err=vk.ReadFrom(reader);must(err);if reader.Len()!=0{panic("trailing verifying-key bytes")}
 proofBytes,err:=os.ReadFile(os.Args[3]);must(err);if len(proofBytes)!=164{panic("proof is not164bytes")};proof:=groth16.NewProof(ecc.BN254);reader=bytes.NewReader(proofBytes);_,err=proof.ReadFrom(reader);must(err);if reader.Len()!=0{panic("trailing proof bytes")}
 publicBytes,err:=os.ReadFile(os.Args[4]);must(err);publicHash:=sha256.Sum256(publicBytes);if hex.EncodeToString(publicHash[:])!=os.Args[5]{panic("public fixture mismatch")};public,err:=witness.New(ecc.BN254.ScalarField());must(err);reader=bytes.NewReader(publicBytes);_,err=public.ReadFrom(reader);must(err);if reader.Len()!=0{panic("trailing public bytes")}
 must(groth16.Verify(proof,vk,public));proofHash:=sha256.Sum256(proofBytes);vkHash:=sha256.Sum256(vkBytes);fmt.Printf("VERIFIED proof_sha256=%x public_sha256=%x vk_sha256=%x image_sha256=%x\n",proofHash,publicHash,vkHash,imageHash)
}
