# Using NextStd

If you have installed NextStd system-wide (via `sudo make install`), you can
include the core header file in any C program using standard angle brackets:

```c
#include <nextstd/ns.h>
```

*Note: If you are just exploring the repository locally and running the built-in
examples, the local headers are automatically linked for you.*

## Type-safe Printing (`ns_print` and `ns_println`)

`NextStd` completely removes the need to memorize `printf` format specifiers.
There are 2 print functions:

- `ns_print()`: Prints it without a newline (`\n`).
- `ns_println()`: A newline (`\n`) is added automatically.

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

Reading user input in standard C usually involves `scanf`, which is notorious
for buffer overflows, leaving dangling newlines in the input stream, and
crashing if the user types the wrong data type.

`ns_read` provides a memory-safe, type-safe alternative. If the user enters an
invalid data type (like typing `hello` when asked for a number), `ns_read`
defaults to `0` instead of panicking. It also gracefully handles `NULL`
pointers.

```c
int main() {
  int age;
  float height;

  ns_print("Enter your age: ");       // No newline 
  ns_read(&age);                      // Automatically routes to ns_read_int 

  ns_print("Enter your height: ");
  ns_read(&height);                   // Automatically routes to ns_read_float 

  ns_print("Age: "); ns_println(age);
  ns_print("Height: "); ns_println(height);

  return 0;
}
```

## Safe strings (`ns_string`)

Standard C strings are notorious for memory leaks and buffer overflows.
`NextStd` uses `ns_string`, which handles its own memory and utilizes
**Small String Optimization** (SSO).

```c
int main() {
  // Strings under 24 bytes are allocated directly on the Stack
  ns_string short_str;
  ns_string_new(&short_str, "Hello");

  // Strings larger than 24 bytes seamlessly move to the heap
  ns_string long_str;
  ns_string_new(&long_str, "This is a long string and it is very long");

  ns_println(short_str);
  ns_println(long_str);

  // Safely drops memory (Preventing double-frees)
  ns_string_free(&short_str);
  ns_string_free(&long_str);

  return 0;
}
```

## Safe Dynamic Arrays (`ns_vec`)

Standard C arrays are fixed-size, and manually managing dynamic arrays with
`malloc` and `realloc` is prone to memory leaks and out-of-bounds access.
`NextStd` provides `ns_vec`, a memory-safe, dynamically resizing array.

To keep compile times fast, data structures are kept in a separate header. You
must explicitly include `ns_data.h`:

```c
#include <nextstd/ns.h>
#include <nextstd/ns_data.h>

int main() {
  ns_vec my_list;

  // Initialize the vector to hold integers
  ns_vec_new(&my_list, sizeof(int));

  // Push values (automatically resizes the heap when capacity is reached)
  ns_vec_push(&my_list, int, 10);
  ns_vec_push(&my_list, int, 20);

  // Safely retrieve values with OOM and out-of-bounds protection
  int val;
  ns_vec_get(&my_list, int, 1, &val);

  ns_print("Value at index 1: ");
  ns_println(val); // Prints 20

  // Safely free the heap memory
  ns_vec_free(&my_list);

  return 0;
}
```

## Safe HashMaps (`ns_map`)

Writing HashMaps in C usually involves implementing complex hashing algorithms
and manually handling memory collisions. `NextStd` provides `ns_map`, backed
by Rust's highly secure `std::collections::HashMap` (which defends against
HashDoS attacks automatically).

Like vectors, this requires the `ns_data.h` header.

```c
#include <nextstd/ns.h>
#include <nextstd/ns_data.h>

int main() {
  ns_error_t err;
  ns_map inventory;

  // 1. Initialize HashMap: Keys are strings, Values are integers
  NS_TRY(err, ns_map_new(&inventory, sizeof(ns_string), sizeof(int))) {
    ns_println("Inventory initialized.");
  } NS_EXCEPT(err, NS_ERROR_ANY) {
    return 1;
  }

  // 2. Create string keys
  ns_string item1, item2, item3;
  ns_string_new(&item1, "Apples");
  ns_string_new(&item2, "Bananas");
  ns_string_new(&item3, "Cherries");

  // 3. Insert items into the map
  ns_map_put(&inventory, ns_string, item1, int, 50);
  ns_map_put(&inventory, ns_string, item2, int, 120);

  // 4. Safely retrieve an existing item
  int stock = 0;
  NS_TRY(err, (ns_map_at(&inventory, ns_string, item1, &stock))) {
    ns_print("Apples in stock: ");
    ns_println(stock);
  } NS_EXCEPT(err, NS_ERROR_ANY) {
    ns_println("Apples not found!");
  }

  // 5. Attempt to retrieve a non-existent item (Safe failure)
  NS_TRY(err, (ns_map_at(&inventory, ns_string, item3, &stock))) {
    ns_println("Cherries found!");
  } NS_EXCEPT(err, NS_ERROR_ANY) {
    ns_println("Cherries not found in inventory. Safe failure!");
  }

  // 6. Cleanup memory
  ns_string_free(&item1);
  ns_string_free(&item2);
  ns_string_free(&item3);
  ns_map_free(&inventory);

  return 0;
}
```
