#include "../ns.h"

int main(void)
{
  float count = 24.45;
  float zero = 0.0;
  float negative = -67.89;

  // Double vars
  double d1 = 789.67;

  // Test Generic Macro
  // Same line
  ns_print(count); ns_print(" ");
  ns_print(negative); ns_print(" ");
  ns_print(zero);

  ns_println("");

  // Newline

  ns_println(count); 
  ns_println(negative); 
  ns_println(zero);

  // Double
  ns_print(d1); ns_print(" ");
  ns_println(d1);

  // Test Raw literal (Double)
  ns_print(345.78); ns_print(" ");
  ns_println(345.78);

  return 0;
}
