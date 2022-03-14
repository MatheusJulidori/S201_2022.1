package main

import(
	"fmt"
)

func main(){

	var a string
	var c int

	fmt.Scan(&a)
	fmt.Scan(&c)

	castA,err := strconv.Atoi(a)

	var sum = castA+c

  if(err) != nil{
    fmt.Println(err)
  } 
  fmt.Println("A soma é: ",sum)
  
  


}
