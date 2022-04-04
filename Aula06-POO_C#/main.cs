using System;

class Program {

  public static void criaArray(int n,int[] array){
    for(int i =0;i<10;i++){
      array[i] = n*i;
    }
    foreach(var item in array){
      Console.Write(item);
    }
  }

  
  public static void Main (string[] args) {
    Console.WriteLine ("Hello World");

    var a = float.Parse(Console.ReadLine());
    var b = float.Parse(Console.ReadLine());
    int soma = (int)a+(int)b;
    Console.WriteLine(soma+" ");

    int[] ab = new int[10];

    criaArray(3,ab);
    
  }


  
}