use std::io;


  //Funções
  fn mostrarNumero(x:i32){
    println!("Valor do número no parâmetro: {}",x);
  }

fn main() {
    println!("Hello, world!");
    let x = 2.0;
    let y:bool = true;
    let mut _num:i32 = 2;
    //mut significa mutavel, ou seja, todas as variaveis são constantes se n declaradas;

    //Tipos numéricos:
    // 8 bits -> i8 =signed u8 = unsigned

    println!("Valor de x = {}",x);

    _num = 4;

    let mut string = String::new();   
    io::stdin().read_line(&mut string).expect("Error reading line");
  //Pode quebrar linha durante a escrita, por ex: io::stdin()
  //                                              .read_line(&mut string)
  //                                              .expect("Error reading line");

    println!("Número lido: {}",string);

  //Converter string para numero
  //trim() -> Corta a string
  //parte() -> Converte para um numero ilegivel
  //Unwrap() -> "Desenrola" o numero

  let convertido:i32 = string.trim().parse().unwrap();
  let res:i32 = convertido*5;
  println!("Número convertido * 5 = {}",res);

  //Concatenar strings

  let string1 = "Alo ";
  let string2 = "galera de cowboy";
  let concatenar = format!("{}{}",string1,string2);
  println!("{}",concatenar);

  //if e else
  let mut n:String = String::new();
  io::stdin().read_line(&mut n);
  let convert:i32 = n.trim().parse().unwrap();
  if convert<0{
    println!("{} = negativo", convert);
  }else if convert > 0{
    println!("{} = positivo", convert);
  }else{
    println!("{} = 0", convert);
  }

  //while
  let mut num3 = 3;
  while num3<101{
    println!("{}",num3);
    num3 *= 3;
  }

  //for 
  let mut num2 = 2;
  for num2 in 1..10{
      println!("{}",num2)
  }

  for num2 in (10..20).step_by(2){
    println!("{}",num2)
  }

  //loop
  let mut num1:i32 = 5;
  loop{
    println!("Que bagulho maneiro");
    if num1==0{
      break;
    }
    num1 -= 1;
  }


  //Funções(Declara no começo)
  mostrarNumero(5);

  //Arrays
  let mut arr:[i32;3] = [0;3];//Inicialia a array com 0 em todas as posições;
  println!("Tamanho do array = {}",arr.len());

  let mut cont:i32 = 0;

  for cont in 0..arr.len(){
    println!("{}",arr[cont]);
  }
  
}