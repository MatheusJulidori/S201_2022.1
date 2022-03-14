package main

import(
	"fmt"
)

func arr(x int)[10]int{
  arr1:=[10]int{}

  for i:=0;i<10;i++{
    arr1[i] = (i+1)*x
  }

  return arr1
}

func main(){

  var x int;
  fmt.Scan(&x)
  arr1 :=[10]int{}

  arr1 = arr(x)

    for i:=0;i<10;i++{
    fmt.Println(arr1[i])
  }
  
  
}
