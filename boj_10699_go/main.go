package main

import (
	"fmt"
	"time"
)

func main() {
	now := time.Now()
	kst, _ := time.LoadLocation("Asia/Seoul")
	now = now.In(kst)
	fmt.Printf("%d-%02d-%02d", now.Year(), now.Month(), now.Day())
}
