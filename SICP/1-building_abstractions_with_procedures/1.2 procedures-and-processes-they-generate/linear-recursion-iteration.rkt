#lang sicp

#| This is a recursive procedure to compute the factorial of a number. |#
(define (factorial n)
    (if (= n 0)
    1
    (* n (factorial (- n 1)))))


(factorial 6)

#| Another way to compute factorial using iteration |#
(define (factorial n)
    (fact-iter 1 1 n))

(define (fact-iter product counter max-count)
    (if (> counter max-count)
        product
        (fact-iter (* counter product)
                   (+ counter 1)
                   max-count)))