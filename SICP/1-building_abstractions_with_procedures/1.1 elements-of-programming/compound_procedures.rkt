#|
procedure definitions, a much more powerful abstraction technique by which a compound operation
 can be given a name and then referred to as a unit.
 |#


 (define (square x) (* x x))

#|
 The procedure represents the operation of multiplying
  something by itself.
  
  Formal definition of a procedure:
|#

(define (procedure-name formal-parameter1 formal-parameter 2) body)


(square 21)
(square (+ 2 5))

; Using square as a building block for other procedures
(define (sum-of-squares x y)
  (+ (square x) (square y)))

; Compound procedures are used in exactly the same way as primitive procedures