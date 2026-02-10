#|
What counts as valid data?

Data is defined by its behavior, not its representation.

Not any arbitrary procedure can represent rational numbers,
they must obey the rules of rational numbers.

Constructor and selectors define the data objects, because
they satisfy the physical properties of the data objects or rules.


Data = operatios + rules operations must satisfy.

|#


(cons x y) ; make a pair
(car z)    ; get first element
(cdr z)    ; get second element

; If this true then we have an valid data abstraction for pairs.
(car z) → x
(cdr z) → y


; Pairs implementation
(define (cons x y)
  (define (dispatch m)
    (cond ((= m 0) x)
          ((= m 1) y)
          (else (error "Argument not 0 or 1: CONS" m))))
  dispatch)

(define (car z) (z 0))
(define (cdr z) (z 1))


; DATA IS DEFINED WHAT YOU CAN DO WITH IT, NOT HOW ITS STORED.


; EXAMPLE
(define (cons x y)
  (define (dispatch m)
    (cond ((= m 0) x)
          ((= m 1) y)
          (else (error "Argument not 0 or 1: CONS" m))))
  dispatch)

(define (car z) (z 0))
(define (cdr z) (z 1))


(define p (cons 10 20))
(car p) ; → 10
(cdr p) ; → 20
