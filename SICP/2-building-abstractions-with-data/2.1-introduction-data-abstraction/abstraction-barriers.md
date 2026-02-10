Each layer of abstraction only needs to know about the layer directly below it.

How pairs are implemented does not matter to someone using rational numbers.


Why abstraction barriers are important

Easier maintenance:

If you want to change the underlying representation (like how rational numbers are stored), you only have to change the bottom layer.

The top layer code stays the same.

Flexibility:

You can experiment with different internal representations without breaking your code.

Example: deciding when to simplify fractions:

At construction: reduce fraction when creating it.

At selection: reduce fraction when you access numerator/denominator.

Either choice works without touching add-rat, mul-rat, etc.

Reduces errors:

You interact only with the interface, so you don’t accidentally rely on internal details.


VISUALIZATION

User code
(add-rat, sub-rat, mul-rat, div-rat)
------------------ abstraction barrier ------------------
Constructors and selectors
(make-rat, numer, denom)
------------------ abstraction barrier ------------------
Data representation
(pairs: cons, car, cdr)