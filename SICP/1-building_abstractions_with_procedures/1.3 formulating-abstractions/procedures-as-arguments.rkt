#lang sicp

(define (sum-integers a b)
  (if (> a b)
      0
      (+ a (sum-integers (+ a 1) b))))

(define (sum-cubes a b)
  (if (> a b)
      0
      (+ (cube a) (sum-cubes (+ a 1) b))))


#|The presence of such a common pattern is strong evidence that there 
is a useful abstraction waiting to be brought to the surface|#


(define (inc n) (+ n 1))

(define (sum-cubes a b)
  (sum cube a inc b))

;Using this, we can compute the sum of the cubes of the integers from 1 to 10: