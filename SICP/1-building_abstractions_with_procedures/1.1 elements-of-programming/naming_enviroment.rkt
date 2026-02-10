#| A critical aspect of a programming language is the means it provides 
for using names to refer to computational objects.|#

#|A critical aspect of a programming language is the means it provides for using names to refer 
to computational objects. We say that the name identifies a variable whose value is the object. |#

#lang sicp

(define size 2)

size

(* 5 size)


(define pi 3.14)
(define radius 10)

(* pi (* radius radius))


#|In general, computational objects may have very complex structures, and it would be extremely 
inconvenient to have to remember and repeat their details each time we want to use them. 
Indeed, complex programs are constructed by building, step by step,
computational objects of increasing complexity.|#
