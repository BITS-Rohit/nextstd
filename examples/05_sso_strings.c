#include "../include/ns.h"

int main()
{
  // String less than 24 bytes (No heap allocation)
  ns_string short_str = ns_string_new("Hello");

  // Long string (Heap allocation)
  ns_string long_str = ns_string_new("This is a much longer string which will be heap allocated automatically");

  // Print them
  ns_print("Short string: ");
  ns_println(short_str);

  ns_print("Long string: ");
  ns_println("long_str");

  // Free memory 
  ns_string_free(&short_str);
  ns_string_free(&long_str);

  return 0;
}
