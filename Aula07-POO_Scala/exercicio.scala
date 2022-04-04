class Veiculo(nomex: String, marcax: String, fabricantex: String, gasolinax: Double) {
   var nome: String = noemx;
   var marca: String = marcax;
   var frabricante: String = fabricantex;
   var gasolina: Double = gasolinax;

   def mostraInfo() {
      Console.println(this.nome +" " + this.marca);
   }
   
   def mostraInfo(nomex: String){
       Console.println(nomex+" "+this.fabricante+" "+this.gasolina);
   }

}

class Carro extends Veiculo(){
    
    
    public Carro(String nomex,String marcax, String fabricantex, double gasolinax){
        super(namex,marcax,fabricantex,gasolinax);
    }
    
    
    private var velocidade: Int = 300;
    
    def getVelocidade(){
        return this.velocidade;
    }
    
    def setVelocidade(velocidadex:Int){
        this.velocidade = velocidadex;
    }
    
}

object Hello {
    def main(args: Array[String]) = {
        var c1 = new Carro("Bolota","Celta","Chevrollet",50)
        c1.mostraInfo();
    }
}

  
