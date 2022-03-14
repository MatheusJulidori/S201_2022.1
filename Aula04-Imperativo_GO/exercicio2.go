package main

import (
	"fmt"
)

func main(){

	fmt.Println("Para sair do loop, entre 0 e 0")
	for{
		var a int
		var c int

		fmt.Scan(&a)
		fmt.Scan(&c)

		if(a == 0  && c == 0){
			break;
		}

		var sum = a+c

		fmt.Println("A soma é: ",sum)
	}

}