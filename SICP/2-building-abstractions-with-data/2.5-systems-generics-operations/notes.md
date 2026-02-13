We want one arithmetic system that works across many kinds of numbers:

normal integers and floats

rational numbers (fractions)

complex numbers

polynomials

And we want to use them together without rewriting everything.

The problem:

Different number types usually live in separate worlds.

This chapter builds a system where all number types plug into a single framework.

Like adding new apps to an operating system.

Core idea

A single generic operation should work for many types.

Instead of:

add-integers
add-rationals
add-complex


we want:

add(x, y)


and the system figures out how to handle it.

This is generic arithmetic.

One practical example — a calculator that grows over time

Imagine building a calculator app.

Version 1 supports:

2 + 3


Easy.

Later you add fractions

Now users want:

1/2 + 3/4


If your calculator is hardcoded for integers:

you rewrite everything.

Bad design.

Later you add complex numbers

Users want:

(3 + 4i) + (1 + 2i)


If your system isn’t modular:

the calculator collapses under its own weight.

This chapter shows how to design it so each number type is a plug-in.

How the system works

Each number type is packaged separately:

integer package

rational package

complex package

polynomial package

Each package registers:

how to add
how to subtract
how to multiply
how to divide


The generic system keeps a table:

operation + types → correct procedure


So when you call:

add(x, y)


the system looks at their types and dispatches to the right code.

This is called data-directed programming.

Why this is powerful

You can add a new number type without touching old code.

Add a polynomial package:

the generic system automatically supports:

add(polynomial, polynomial)
mul(polynomial, polynomial)


No rewriting required.

That’s additive design.

Large systems depend on this idea.

The hard part — mixing types

Now comes the real problem.

What should happen when users do:

3 + (4 + 2i)


An integer plus a complex number.

They are different types.

Who handles that?

Coercion (type conversion)

The system solves this by converting types.

Example:

3 → 3 + 0i


Now both values are complex.

The complex package handles the operation.

This conversion is called coercion.

Instead of writing special rules for every pair of types,
we teach the system how to convert between types.

Then it reuses existing logic.

This avoids combinatorial explosion of cases.

Type hierarchies (the tower idea)

Many numeric systems form a natural ladder:

integer → rational → real → complex


Each level can be seen as a more general version of the previous one.

The system can “raise” numbers upward:

2 → 2/1 → 2.0 → 2 + 0i


When mixing types:

everything gets raised to the highest level.

Then operations are safe.

This is like promoting types in modern languages.

Polynomial example (why abstraction matters)

Polynomials are built from numbers:

3x² + 2x + 7


The coefficients (3, 2, 7) are numbers.

But those numbers could themselves be:

integers

rationals

complex numbers

even other polynomials

Because polynomial arithmetic uses generic add/mul,
the system automatically works with all of them.

No extra code.

The abstraction handles recursion naturally.

That’s the big win.

Big picture takeaway

This chapter is about building a universal arithmetic framework:

new number types plug in

old code stays untouched

mixed operations work safely

the system scales without chaos

It’s a blueprint for extensible systems.

Not just math.

Architecture.

One sentence summary

Generic operations let many independent modules behave like one unified system.

That’s why these ideas were invented.