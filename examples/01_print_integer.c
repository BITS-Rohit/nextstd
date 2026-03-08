#include "../include/ns.h"

int main() {
  int count = 42;
  int negative = -100;
  int zero = 0;

  // Test Generic Macro (No Newline)
  ns_print(count); ns_print(" ");
  ns_print(negative); ns_print(" ");
  ns_print(zero); 

  ns_println(" ");
  // Newline
  ns_println(count);
  ns_println(negative);
  ns_println(zero);


  // Test Raw Literal
  ns_print(12345); ns_print(" ");
  ns_println(12345);

  return 0;
}
