#lang sicp

(define (cube x) (* x x x))
#|here we are not talking about a cube of a particular number
but rather a method for obtaining the cube of any number |#

; if language doesnt allow the construction of absstractions
; we would be stuck with the following code using primitives
(* 3 3 3)
(* x x x)
(* y y y)

#|One of the things we should demand from a powerful programming
language is the ability to build abstractions by assigning names
to common patterns and then to work in terms of the abstractions
directly. Procedures provide this ability.|#