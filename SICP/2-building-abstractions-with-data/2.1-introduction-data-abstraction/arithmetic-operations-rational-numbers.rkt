
#lang sicp 

; Defining what a rational numbers is and how to construct it

; GOOD ABSTRACTION ISOLATES CHANGES
; INTERNAL REPRESENTATION CAN CHANGE SAFELY
(define (make-rat n d)
    (let ((g (gcd n d)))
      (cons (/ n g) (/ d g))))

(define (numer x)
  (car x))
(define (denom x)
  (cdr x))


(define (add-rat x y)
  (make-rat (+ (* (numer x) (denom y))
               (* (numer y) (denom x)))
            (* (denom x) (denom y))))

(define (sub-rat x y)
  (make-rat (- (* (numer x) (denom y))
               (* (numer y) (denom x)))
            (* (denom x) (denom y))))

(define (mul-rat x y)
  (make-rat (* (numer x) (numer y))
            (* (denom x) (denom y))))

(define (div-rat x y)
  (make-rat (* (numer x) (denom y))
            (* (denom x) (numer y))))

(define (equal-rat? x y)
  (= (* (numer x) (denom y))
     (* (numer y) (denom x))))



; all arithmetic is writtien without knowing how 
; the rational numbers are stored.


;Arithmetic code does not care how numbers are represented.

(define x (cons 1 2))