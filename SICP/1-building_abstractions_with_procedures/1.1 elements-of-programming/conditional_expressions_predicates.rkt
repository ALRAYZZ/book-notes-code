; Conditional procedure


(define (abs x)
  (cond ((> x 0) x)
        ((= x 0) 0)
        ((< x 0) (- x))))


#|
The general form of a conditional expression is

(cond (⟨p₁⟩ ⟨e₁⟩)
      (⟨p₂⟩ ⟨e₂⟩)
      …
      (⟨pₙ⟩ ⟨eₙ⟩))


The first expression in each pair is a predicate, that is,
an expression whose value is interpreted as either true or false.    


Conditional expressions are evaluated as follows. The predicate ⟨p1⟩
 is evaluated first. If its value is false, then ⟨p2⟩
 is evaluated. If ⟨p2⟩
’s value is also false, then ⟨p3⟩
 is evaluated. 
|#

(define (abs x)
  (cond ((< x 0) (- x))
        (else x)))


#|
In addition to primitive predicates such as <, =, and >,
there are logical composition operations, 
which enable us to construct compound predicates. 
he three most frequently used are these:

(and ⟨e₁⟩ … ⟨eₙ⟩)
The interpreter evaluates the expressions ⟨e⟩ one at a time,
in left-to-right order. If any ⟨e⟩ evaluates to false, 
the value of the and expression is false, and the rest of the ⟨e⟩’s
are not evaluated. If all ⟨e⟩’s evaluate to true values,
the value of the and expression is the value of the last one.

(or ⟨e₁⟩ … ⟨eₙ⟩)
The interpreter evaluates the expressions ⟨e⟩ one at a time,
in left-to-right order. If any ⟨e⟩ evaluates to a true value,
that value is returned as the value of the or expression, 
and the rest of the ⟨e⟩’s are not evaluated. If all ⟨e⟩’s evaluate to false,
the value of the or expression is false.

(not ⟨e⟩)
The value of a not expression is true when the expression ⟨e⟩ evaluates to false,
and false otherwise.
|#