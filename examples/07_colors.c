#include "../include/ns.h"
#include "../include/ns_color.h"

int main()
{
  ns_println("Printing in colors");
  ns_println(NS_COLOR_GREEN "Green" NS_COLOR_RESET);

  ns_error_t err;

  int age;

  ns_print(NS_COLOR_YELLOW "Enter age: " );

  NS_TRY(err, ns_read(&age)) {
    ns_print(NS_COLOR_RESET);
    ns_print(NS_COLOR_GREEN "Success, you entered : ");
    ns_println(age);
    ns_print(NS_COLOR_RESET);
  }
  NS_EXCEPT(err, NS_ERROR_INVALID_INPUT) {

    // Yellow for a user input warning
    ns_println(NS_COLOR_RED "That is not a valid number! Please enter digits only." NS_COLOR_RESET);

  } 
  NS_EXCEPT(err, NS_ERROR_IO_READ) {

    ns_println(NS_COLOR_RED "Failed to read from standard input." NS_COLOR_RESET);

  } 
  NS_EXCEPT(err, NS_ERROR_ANY) {

    ns_print(NS_COLOR_RED "An error occurred: ");
    ns_println(ns_error_message(err));
    ns_print(NS_COLOR_RESET);

  }
}
