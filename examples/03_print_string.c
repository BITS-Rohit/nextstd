#include "../ns.h"

int main(void)
{
  char* str = "Hello World";

  // Print string 
  ns_print("String (Same line): ");
  ns_println(str);

  // Print raw string literal
  ns_print("Raw String Literal"); ns_print(" ");
  ns_println("Raw String Literal");

  return 0;
}
