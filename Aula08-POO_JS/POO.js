class Animal {
    constructor(nome,idade,especie){
        this.nome = nome;
        this.idade = idade;
        this.especie = especie;
    }

    printInfo(){
        console.log(this.nome)
        console.log(this.idade)
        console.log(this.especie)
    }
}

class Cachorro extends Animal {
    constructor(nome,idade,especie,raca){
        super(nome,idade,especie)
        this.raca = raca;
    } 

    printInfo(){
        super.printInfo()
        console.log(this.raca)
    }
}

class Gato extends Animal {
    constructor(nome,idade,especie,cores){
        super(nome,idade,especie)
        this.cores = cores
    }

    printInfo(){
        super.printInfo()
        this.cores.forEach(cor => console.log(cor))
    }
}

let animal = new Animal("Animal",21,"Animal")
animal.printInfo()

let dog = new Cachorro("Dog",21,"Dog","Bacet")
dog.printInfo()

let gato = new Gato("Gato",21,"Gato",["Preto","Branco","Amarelo"])
gato.printInfo()
