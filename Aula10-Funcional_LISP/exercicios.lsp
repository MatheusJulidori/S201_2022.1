//Exercicio 1

(
    defun quadrado (x)
    (* x x)
)

(print(quadrado 2))

//Exercicio 2

(defparameter my-list (list 1 2 3))
(defparameter my-list2 (list 4 5 6))

(
    defun exercicio (x)
    (
        if (>= x 4)
        (* x 2)
        (/ x 2)
    )
)

(defparameter x (mapcar #'exercicio my-list))
(defparameter y (mapcar #'exercicio my-list2))

(defparameter z (append x y))

(print z)

//exericio 3



