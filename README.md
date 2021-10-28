pool
=================
An ergonomic, `befunge` and `<><` inspired language.
-----------------

# 1. The Language
`pool` is a two-dimensional language. Unlike boring and limited languages, `pool` can escape the boring realm of linearity and enter planar space. Whilst `pool` is mainly stack-based, it provides other modern features such as memory allocation on a heap(? i guess, idk).
`pool` targets advanced programmers who strive both speed of execution as well as ease of use.
We at `pool` team:tm:® have not yet proven that `pool` is turing complete but it probably is.

 
# 2. The Instructions
| instruction     | meaning                                                                                                                                                                                                                       |
| --------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `v`             | change the instruction pointer's direction downwards, if it is moving horizontally                                                                                                                                            |
| `^`             | change the instruction pointer's direction upwards, if it is moving horizontally                                                                                                                                              |
| `<`             | change the instruction pointer's direction to go left, if it is moving vertically                                                                                                                                             |
| `>`             | change the instruction pointer's direction to go right, if it is moving vertically                                                                                                                                            |
| `+`             | pop two values off the stack and push their sum                                                                                                                                                                               |
| `-`             | pop two values a,b off the stack and push their difference b-a                                                                                                                                                                |
| `*`             | pop two values off the stack and push their product                                                                                                                                                                           |
| `/`             | pop two values a,b off the stack and push their quotient b/a                                                                                                                                                                  |
| `%`             | pop two values a,b off the stack and push their quotient b%a                                                                                                                                                                  |
| `0-9` and `a-f` | push the value of the literal in hex to the stack                                                                                                                                                                             |
| `,`             | pop the top value of the stack and print it (all 8 bytes)                                                                                                                                                                     |
| `"`             | enter string mode, in where `"` will end string mode. In string mode, a backslash `\` encodes a *newline* when followed by `n` , *tab* when followed by `t`, *carriage return* when followed by `r`, *"* when followed by `"` |
|                 |
| `;`             | pop the top value of the stack and exit with it as exit code                                                                                                                                                                  |
| `$`             | drop the top value on the stack                                                                                                                                                                                               |
| `&`             | swap the top two values of the stack                                                                                                                                                                                          |
| `|`             | if moving horizontally, pop the top value on the stack. If it is unequal to zero, the pointer is mirrored and now will be going in the other direction                                                                        |
| `_`             | if moving vertically, pop the top value on the stack. If it is unequal to zero, the pointer is mirrored and now will be going in the other direction                                                                          |
| `s`             | pop two values `address`,`value` off the stack and set the heap at `address` to `value`                                                                                                                                       |
| `r`             | pop a single word `address` off the stack and push the value of the heap at `address` to the stack |
| `n`             | pop a single value off the stack and push the square root of it (yeah) |
| `n` or `√`             | pop a single value off the stack and push the square root of it (yeah) (n because √ didnt work in c++, v is already used, b too so n it is)|
