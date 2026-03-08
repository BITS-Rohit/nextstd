# Using NextStd

Include the `ns.h` header file in your C program. If you are trying out the
example programs, linking to all the other headers are done automatically.
If not, take a look at `Makefile` to get an idea on how to link `ns.h` for
linking syntax.

```c
#include "include/ns.h"
```

## Type-safe Printing (`ns_print` and `ns_println`)

`NextStd` completely removes the need to memorize `printf` format specifiers.
There are 2 print functions :

- `ns_print()`: Prints it without newline (`\n`).
- `ns_println()` : Newline (`\n`) is added automatically.

```c
int main() {
  int age = 42;
  double pi = 3.14159;

  // Type detected automatically 
  ns_println(age);
  ns_println(pi);

  return 0;
}
```

## Safe user input (`ns_read`)

Reading user input in standard C usually involves `scanf`, which is notorious for
buffer overflows, leaving dangling newlines in the input stream and crashing
if the user types the wrong data type.

`ns_read` provides a memory-safe, type-safe alternative. If the user enters
an invalid data type (Like typing `hello` when asked for a number), `ns_read`
defaults to `0` instead of panicking. It also gracefully handles `NULL` pointers.

```c
int main() {
  int age;
  float height;

  ns_print("Enter your age: ");       // No newline 
  ns_read(&age);                      // Automatically routes to ns_read_int 

  ns_print("Enter your height: ");
  ns_read(&height);                  // Automatically routes to ns_read_float 

  ns_print("Age: "); ns_println(age);
  ns_print("Height: "); ns_println(height);

  return 0;
}
```

## Safe strings (`ns_string`)

Standard C strings are notorious for memory leaks and buffer overflows. `NextStd`
uses `ns_string`, which handles it's own memory and utilizes
**Small String Optimization** (SSO)

```c
int main() {
  // Strings under 24 bytes are allocated directly on the Stack
  ns_string short_str = ns_string_new("Hello");

  // Strings larger than 24 bytes seamlessly move to the heap
  ns_string long_str = ns_string_new("This is a long string and it is very long");

  ns_println(short_str);
  ns_println(long_str);

  // Safely drops memory (Preventing double-frees)
  ns_string_free(&short_str);
  ns_string_free(&long_str);

  return 0;
}
```

> [!IMPORTANT]
> `ns_read()` cannot handle `ns_string()` for now. This will be added in future versions
