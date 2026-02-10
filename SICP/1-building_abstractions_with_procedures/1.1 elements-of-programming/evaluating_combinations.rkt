#|To evaluate a combination, do the following:
Evaluate the subexpressions of the combination.
Apply the procedure that is the value of the leftmost subexpression (the operator) to the arguments
 that are the values of the other subexpressions (the operands).|#

 (* (+ 2 (* 4 6)) (+ 3 5 7))

#|the evaluation rule is recursive in nature; that is, it includes, as one of its steps,
 the need to invoke the rule itself|#

 