class Pessoa:
    def __init__(self,nome,idade):
        self.nome = nome
        self.idade = idade
    
    
p1 = Pessoa("Matheus", "21")
p2 = Pessoa("Arthur", "21")

print(p1.nome +" "+ p1.idade)

print(p2.nome+" "+  p2.idade)
