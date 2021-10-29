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
| `!`             | pop the top value on the stack and if it is 0 push 0 else push 1                                                                                                                                                              |
| \`       | pop two values a,b off the stack and push 1 if b>a else push 0 |
| `&`             | swap the top two values of the stack                                                                                                                                                                                          |
| `|`             | if moving horizontally, pop the top value on the stack. If it is unequal to zero, the pointer is mirrored and now will be going in the other direction                                                                        |
| `_`             | if moving vertically, pop the top value on the stack. If it is unequal to zero, the pointer is mirrored and now will be going in the other direction                                                                          |
| `s`             | pop two values `address`,`value` off the stack and set the heap at `address` to `value`                                                                                                                                       |
| `r`             | pop a single word `address` off the stack and push the value of the heap at `address` to the stack                                                                                                                            |
| `n`             | pop a single value off the stack and push the square root of it (yeah)                                                                                                                                                        |
| `n` or `√`      | pop a single value off the stack and push the square root of it (yeah) (n because √ didnt work in c++, v is already used, b too so n it is)                                                                                   |





# 3. Getting Started

## 3.1 Installation
Build from source: make sure [rust](https://www.rust-lang.org/tools/install) is installed, then run 
```bash
cargo build --release
```
the binary will be in `./target/release/pool-lang`

## 3.2 Tutorial
create a new file called tutorial.2d or clone [tutorial.2d](https://github.com/HiddyTiddy/pool-lang/blob/main/programs/tutorial.2d)

every program should start with this. The instruction pointer will start at `.` and move in `>` direction. In fact, `.` alone is equivalent to this but I think `.>` is more idiomatic
```pool
.> 
```

Let's write a first Hello Pool program. It is very easy actually
```pool
.> "Hello Pool!\n",,,,,,,,,,,,0;
```
but this a) is ugly and b) prints out 
```

!looP olleH
```
that is because pool is stack based, which is an LIFO (last in first out) data structure. To mend this, we can change the program to read 
```pool
.> "\n!looP olleH" ,,,,,,,,,,,,0;
```

Now with that knowledge we can do loops. A basic loop looks like this.
```pool
.>  ff*0 > 1+ oo &` v

         ^ ,"!"   | < "\n",0;
```
Here, first a maximum value is pushed (`ff*` = 15 * 15 = 225). On the stack, we also need the counter which we initialize with 0. It is then incremented by 1. Then the maximum and the counter are duplicated, swapped and compared, such that if the counter is greater than the maximum it pushes 1 to the stack, leading the pointer to be reflected by `|`.