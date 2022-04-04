class Carro(modelo: String, marca: String) {
   var modelo: String = modelo;
   var marca: String = marca;
   var velocidade: Int = 300;

   def andar(metros: Int) {
      Console.println(this.modelo+" andou" + this.velocidade +" metros")
   }

   //Getter e setter e encapslaento é igual
}