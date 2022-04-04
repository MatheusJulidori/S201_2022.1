class Carro(modelox: String, marcax: String) {
   var modelo: String = modelox;
   var marca: String = marcax;
   var velocidade: Int = 300;

   def andar(metros: Int) {
      Console.println(this.modelo+" andou " + metros +" metros na velocidade de "+this.velocidade+"km/h");
   }
 //Getter e setter e encapslaento é igual

}


var c1 = new Carro("Celta", "Chevrollet");
c1.andar(25000);
  
