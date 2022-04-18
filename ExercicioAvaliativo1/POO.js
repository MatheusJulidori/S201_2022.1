class Televisao {
    constructor(modelo){
        this.modelo = modelo;
        this.canal = null;
        this.volume = 0
    }

    printInfo(){
        console.log(this.modelo)
        console.log(this.volume)
        console.log(this.canal)
    }

    aumentaVolume(valor){
        this.volume+=valor
    }

    diminuiVolume(valor){
        this.volume-=valor
    }

    trocaCanal(canal){
        this.canal = canal
    }
}

let tv = new Televisao("LG")

tv.aumentaVolume(70)
tv.diminuiVolume(27)
tv.trocaCanal("Canal #1")
tv.printInfo()

