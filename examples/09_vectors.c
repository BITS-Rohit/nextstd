#include "../include/ns.h"
#include "../include/ns_data.h"
#include "../include/ns_color.h"

int main() {
  ns_error_t err;
  ns_vec my_list;

  ns_println(NS_COLOR_CYAN NS_COLOR_BOLD "---- NextStd Safe Vector Example ----" NS_COLOR_RESET);

  // 1. Initialize the vector to hold standard integers
  NS_TRY(err, ns_vec_new(&my_list, sizeof(int))) {
    ns_println(NS_COLOR_GREEN "Vector initialized successfully!" NS_COLOR_RESET);
  } NS_EXCEPT(err, NS_ERROR_ANY) {
    ns_println(NS_COLOR_RED "Failed to initialize vector." NS_COLOR_RESET);
    return 1;
  }

  // 2. Push items into the vector
  // Our initial capacity is 4. Pushing 10 items will force the Rust backend 
  // to safely resize the heap memory multiple times!
  ns_println("\nPushing 10 items into the vector...");
  for (int i = 0; i < 10; i++) {
    int value = i * 10; // 0, 10, 20, 30...

    if (ns_vec_push(&my_list, int, value) != 0) {
      ns_println(NS_COLOR_RED "Failed to push item! Heap allocation failed." NS_COLOR_RESET);
      break;
    }
  }

  // Print the internal tracking stats to prove it grew
  ns_print("Current Length: ");
  ns_println((int)my_list.len);
  ns_print("Current Capacity: ");
  ns_println((int)my_list.capacity);

  // 3. Safely retrieve the items
  ns_println(NS_COLOR_CYAN "\nRetrieving items safely:" NS_COLOR_RESET);
  for (size_t i = 0; i < my_list.len; i++) {
    int val;
    // The macro does the bounds checking and type-casting automatically
    if (ns_vec_get(&my_list, int, i, &val) == 0) {
      ns_print("Index "); ns_print((int)i); ns_print(": ");
      ns_println(val);
    }
  }

  // 4. Test Out-of-Bounds Protection (The true test of safety)
  ns_println(NS_COLOR_YELLOW NS_COLOR_BOLD "\n---- Testing Out-of-Bounds Safety ----" NS_COLOR_RESET);

  int bad_val;
  // Attempting to read index 50 (we only pushed 10 items)
  ns_println("Attempting to read index 50...");
  if (ns_vec_get(&my_list, int, 50, &bad_val) != 0) {
    ns_println(NS_COLOR_GREEN "Success! The macro caught the out-of-bounds read and blocked it." NS_COLOR_RESET);
  } else {
    ns_println(NS_COLOR_RED "CRITICAL FAILURE: The macro allowed an out-of-bounds read!" NS_COLOR_RESET);
  }

  // 5. Clean up the memory to prevent leaks
  ns_vec_free(&my_list);
  ns_println(NS_COLOR_GREEN "\nMemory successfully freed. Exiting safely." NS_COLOR_RESET);

  return 0;
}
