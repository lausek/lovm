# what is `lausek's own virtual machine`?

we want our virtual machine to be:

- simple
    - allow straight-forward interaction on frontend/il/bytecode
    - use appealing syntax for api/il

- hacky
    - do what you want, have precise control over the process
    - but *secure* e.g. exploit of program doesn't lead to exploit of host

# what does this intend?

- supply a versatile base for languages

# how are things done the `lovm` way?

## paradigm

`lovm` clearly separates values, functions, and types

we want to think about functions first. object-methods are supported by dynamic-dispatch over the supplied function signature.

## memory

memory management must be done manually.

## error handling

try-catch

## threading

use interrupt to spawn/join

## interrupts

... are a cheap way of extending the functionality of `lovm`
