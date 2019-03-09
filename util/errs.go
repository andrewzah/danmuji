package util

import (
	"fmt"
)

func ErrDebug(msg string, err error) {
	if err != nil {
		fmt.Printf("%s: %+v", msg, err)
	}
}

func ErrPanic(msg string, err error) {
	if err != nil {
		fmt.Printf("%s: %+v", msg, err)
		panic(err)
	}
}
