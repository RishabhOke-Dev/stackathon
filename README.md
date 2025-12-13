# Stackathon Language

`Stackathon` is a stack based language, similar to Forth.

## Basic Syntax

Comments sit between two `;`.

Any numbers, strings, or functions are immediatly pushed to the stack.

```stackathon
2 2 "hello" my_function
;Stack is now [2, 2, "hello", my_function];
```
The operators '+', '-', '/', and '*' work on the top two items on the stack.
```stackathon
2 2 +
;The + operator adds 2 + 2;
```
When using the `/` with a string and an int, lets say n, the nth character in the string is pushed.

```stackathon
"Hello" 2 / print
;Prints 'e';
```

Use the print keyword to print the top of the stack.
Currently, when printing blocks, the output is not as clean as the other types.
```stackathon
2 3 * print
;prints 6;
```
## Functions
In `stackathon`, there are two types of functions. Names functions, and blocks, or anonymous functions.
To define a named function, use the `@` operator, with the body in `{}`.
```stackathon
@foo {
    2 +
    ;Adds two to its input;
}
```
To define a block, simply put the body in `{}`, and they will be pushed to the stack
``` stackathon
{
    2 2 + print
}
;Stack now contains the block;
```
To call either of these, use the `$` operator.
The `$` operator runs any function on the top of the stack.
``` Stackathon
@foo {
    2 +
}

100 foo $ print ;Prints 102;

3
{
    2 *
}
$
print
;Prints 6;

```
To exit a function early, use the `exit` keyword.
## Control Flow
### Loops
In order to make a loop, use the `loop` keyword.
The `loop` keyword acts similarly to a while loop in other langauges.

The `loop` keyword expects a function on the top of the stack, and an initial condition below it.
The function is run every cycle. Unlike a while loop, the initial condition, is dropped after its first use.
The function is required to put a boolean on the top of the stack every time it finishes.

```stackathon
15 ;15 is the iteration amount; true {
    "one loop" print
    - 1 ;Decrease iter amount;
    0 = !;Check if it is done, if not then we continue, so we also negate it;
} loop
```
This behavior allows the `loop` keyword, to be made into many different types of loops.
### Gates
A gate is the same as an if else gate in other languages.

The `gate` keyword requires the true function on the top of the stack, an optional false block below it, and a condition below it.

```stackathon
5 4 < {
    "true!" print
}
gate
;prints nothing;
```
## Types and Tags
In `stackathon`, you can check the type of a value using the `type` keyword. Here is a list of all the types and examples of them.
* `int` eg. 5
* `float` eg. 5.1
* `string` eg. "Hello"
* `bool` eg. `true`
* `block` eg. A function or a block
Tags are functions with no bodies, you define them like `@name`. They are used for custom types, and can be pushed by writing out their name.
```stackathon
@int

2 type int = ;The type keyword pushes a tag!;
```
## Libraries
In `stackathon`, libraries are actually compiled! Any named functions and tags, are serialized into a `.stk.lib` file. In order to tell the interpreter to treat your `.stk` as a library, use the argument `--lib` after the name of your file. In order to use a library, use the     `use` keyword. After the `use` keyword, put the path to your library. After the keyword, you can use any function that is available in the function.
``` stackathon
;my_lib.stk;

@foo {
    "foo" print
}

```
```stackathon
;my_program.stk;

use my_lib

foo $
;Works!;
```
## Keywords
* `print` Prints the top of the stack
* `true` Pushes boolean true to the top of the stack
* `false` Pushes boolean false to the top of the stack
* `exit` Exits a function early
* `loop` Used to make loops
* `gate` Used to make if else Gates
* `dup` Duplicates top of the stack
* `drop` Deletes top of the stack
* `swap` Swaps top two values on the stack
* `depth` Pushes length of stack
* `rot` Puts 3rd item of the stack on the top
* `nrot` Puts top of the stack into the 3rd space on the stack
* `over` Duplicates the 2nd item of the stack
* `tuck` Duplicates the top item, and puts it under the second item, (in the third slot of the stack)
* `pick` Copies the nth item of the stack to the top, where n is on the top of the stack when it is called
* `roll` Pulls the nth item of the stack to the top, where n is on the top of the stack when it is called
* `clear` Clears the entire stack
* `type` Pushes type of the top of the stack onto the stack.
* `use` Invokes a stackathon library.
* `input` Pushes the user input as a string
* `strlen` Pushes the length of a string, in Unicode Scalar values.

## Future Features
* Macros, to simplify code
