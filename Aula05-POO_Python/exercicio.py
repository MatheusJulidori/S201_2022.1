class Pessoa:

    def __init__(self, nome,idade):
        self.nome = nome
        self.idade = idade


class Professor(Pessoa):

    def __init__(self,nome,idade):
        self.nome = nome
        self.idade = idade

    def print_idade(self):
        print(f'Professor de idade {self.idade}')

class Aluno(Pessoa):

    def __init__(self,nome,idade,matricula):
        self._matricula = matricula;
        self.nome = nome
        self.idade = idade
    
    def print_idade(self):
        print(f'Aluno de idade {self.idade}')

def chama_idade(obj):
    obj.print_idade()

p1 = Pessoa("Matheus",21)
pr1 = Professor("Arthur",21)
a1 = Aluno("Manuela",20,670)

chama_idade(a1)
chama_idade(pr1)



