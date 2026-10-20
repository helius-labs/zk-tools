//go:build !darwin && !linux

package main

func footprintMB() uint64 { return 0 }
