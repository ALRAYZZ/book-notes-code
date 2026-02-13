Programs break when they depend directly on how data is stored.

As software grows:

storage formats change

performance needs change

new features are added

If the whole program touches raw storage, every change becomes a disaster.

This chapter teaches how to design systems that can change safely.

Core idea

Programs should depend on how to use data, not how it is stored.

We hide storage details behind a stable interface.

That interface protects the rest of the system.

Single practical example — User accounts

Imagine a program that manages user accounts.

At first, accounts are stored like this:

["Alice", "alice@email.com", 100]


Balance is at index 2.

If the program directly reads:

account[2]


then every part of the system depends on this storage layout.

Later the storage changes

You upgrade to:

{
  name: "Alice",
  email: "alice@email.com",
  balance: 100
}


Now every line that used [2] breaks.

You must search the entire codebase.

This is fragile design.

Abstraction solution

Instead, the program uses:

getBalance(account)


Only getBalance knows the storage details.

When storage changes:

you rewrite one function.

Everything else keeps working.

That protective layer is data abstraction.

Why multiple representations matter

Sometimes there are two good ways to store the same data.

We want to support both without rewriting the program.

The chapter shows how to:

label data with a type tag

choose behavior based on the tag

plug in new representations

extend systems without editing old code

This allows multiple implementations to coexist.

Additive system design

The goal is:

Add new features without rewriting old code.

Large systems survive only if they are additive.

Data-directed programming builds a structure where:

new representations plug in

old code stays untouched

modules remain independent

This is professional system architecture.

Message passing (modern connection)

Another approach is to let data objects respond to messages.

Instead of functions deciding what to do:

objects decide for themselves.

This idea leads directly to:

object-oriented programming

classes

methods

encapsulation

Modern languages are built on this principle.

Big picture takeaway

This chapter is about designing software that can grow without collapsing.

It teaches how to:

protect systems from change

support multiple implementations

build modular architecture

extend systems safely

One sentence summary

Hide storage details behind stable interfaces so programs can evolve without breaking.

That is the reason these ideas exist.