import scala.io.StdIn.readLine

object Main{
    def main(args: Array[String]) = {
        

    print("Entre com o primeiro numero: ")
    var firstNumber = scala.io.StdIn.readInt();

    print("Entre com o último numero: ")
    var lastNumber = scala.io.StdIn.readInt();

    
    var arr = new Array[Integer](10)

    for(i <-0 to arr.length-1)
        if(i%2==0){
            arr(i) = i*firstNumber
        }else{
            arr(i) = i*lastNumber
        }
    }
    for(i <- 0 to arr.length-1){
        Console.println(arr)
    }
}