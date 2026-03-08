#include "../include/ns.h"

int main(void)
{
  int age;
  double pi;

  // Read int 
  // Same line
  ns_print("Enter your age: ");
  ns_read(&age);

  // Read Double 
  ns_print("Enter value of PI: ");
  ns_read(&pi);

  // Newline 
  ns_println("-----RESULTS------");
  ns_print("Age: ");
  ns_println(age);

  ns_print("Pi: ");
  ns_println(pi);
}
