#include "../include/ns.h"
int main(void)
{
  ns_error_t err;
  ns_string text;

  ns_print("Enter a string: ");
  NS_TRY(err, ns_read(&text)) {
    ns_print("Text Entered: ");
    ns_println(text);
  }
  NS_EXCEPT(err, NS_ERROR_ANY) {
    ns_println(ns_error_message(err));
  }
}
