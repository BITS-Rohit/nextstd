#include "../include/ns.h"

int main(void)
{
  ns_error_t err;
  ns_string greeting;

  NS_TRY(err, ns_string_new(&greeting, "Hello! This is a string")) {
    ns_print("The string is: ");
    ns_println(greeting);
    ns_print("Length of string: ");
    ns_println(greeting.len);
  }
  NS_EXCEPT(err, NS_ERROR_ANY) {
    ns_println(ns_error_message(err));
  }

  ns_string_free(&greeting);

  return 0;
}
