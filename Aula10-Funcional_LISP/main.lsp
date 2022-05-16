//Definir func que soma

(
    defun sum (a,b,c)
    (+ a,b,c)
)


//declarar var

(defparameter x 3)
(print x)


//Definindo uma lista
(list 1 2 3)

//salvand lista

(defparameter my-list (list 1 2 3))

(print (nth 2 my-list)) -> printa segundo elemento

/loop

(loop for x in (1 2 3)
    do (print x)
)

(loop repeat 10)
    do (print "printando")

/condição

(
    if (> 4 5)
    (message "4<5")
    (message "4>5)
)

//Aplicar func sobre lista
mapcar#'func lista