package main

import (
 "bytes"
 "crypto/sha256"
 "encoding/hex"
 "os"
 "path/filepath"
 "testing"
 "github.com/consensys/gnark/backend/groth16"
 "github.com/consensys/gnark/backend/witness"
 "github.com/consensys/gnark-crypto/ecc"
)
func TestRejectMalformedImage(t *testing.T){for _,data:=range [][]byte{nil,make([]byte,23),make([]byte,24),append([]byte("zopro-euler-v2\x00\x00"),bytes.Repeat([]byte{255},8)...)}{func(){defer func(){if recover()==nil{t.Error("malformed image accepted")}}();verifyingKey(data)}()}}
func TestPinnedImageKnownProofAndNegativeControls(t *testing.T){
 image,e:=os.ReadFile(os.Getenv("TEST_ZOPRO_IMAGE"));if e!=nil{t.Fatal(e)};h:=sha256.Sum256(image);if hex.EncodeToString(h[:])!="16a3e2a686372d531879d4ff99e63eab0ca374d686f05a9d7343d2bf534dd041"{t.Fatal("wrong image")};vk:=groth16.NewVerifyingKey(ecc.BN254);if _,e=vk.ReadFrom(bytes.NewReader(verifyingKey(image)));e!=nil{t.Fatal(e)}
 p:=os.Getenv("TEST_ZOPRO_PROOF");proofBytes,e:=os.ReadFile(p);if e!=nil{t.Fatal(e)};publicBytes,e:=os.ReadFile(filepath.Join(filepath.Dir(p),"0.public"));if e!=nil{t.Fatal(e)};pub,e:=witness.New(ecc.BN254.ScalarField());if e!=nil{t.Fatal(e)};if _,e=pub.ReadFrom(bytes.NewReader(publicBytes));e!=nil{t.Fatal(e)};proof:=groth16.NewProof(ecc.BN254);if _,e=proof.ReadFrom(bytes.NewReader(proofBytes));e!=nil{t.Fatal(e)};if e=groth16.Verify(proof,vk,pub);e!=nil{t.Fatal(e)}
 bad:=append([]byte(nil),proofBytes...);bad[17]^=1;proof=groth16.NewProof(ecc.BN254);_,e=proof.ReadFrom(bytes.NewReader(bad));if e==nil&&groth16.Verify(proof,vk,pub)==nil{t.Fatal("altered proof accepted")}
}
