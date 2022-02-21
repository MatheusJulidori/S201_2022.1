x = 3
print(x)

print(y)

print("Entre com y: ")
y = io.read()


  
print((y*x))

  
print('aaaa'..'bbbbb')

print('aaaa'.. y)

  y = tonumber(y)

if (y>4) then 
    print("Maior que 4")
else if(y<4) then 
    print("Menor que 4")
    else
    print("Igual a 4")

    end
end

i = 0
  while i<5 do
    print(i)
    i = i+1
    end

linha = ' '
repeat
    print(linha)
    linha = io.read()
until linha == "sair"

for i = 1,10 do
    print(i)
end

function factorial(n)
    if (n == 0) then
        return 1
    else
        return n * factorial(n - 1)
    end
end

for n = 0, 16 do
    io.write(n, "! = ", factorial(n), "\n")
end

      
a = {}
a[1] = 2
a[2] = 1
print("a[1] = ", a[1])