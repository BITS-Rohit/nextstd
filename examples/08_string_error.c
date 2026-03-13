#include "../include/ns.h"
#include "../include/ns_error.h"
#include "../include/ns_color.h"

int main() {
  ns_error_t err;
  ns_string str1;
  ns_string str2;
  ns_string result;

  ns_println(NS_COLOR_CYAN NS_COLOR_BOLD "---- NextStd Safe String Example ----" NS_COLOR_RESET);

  // 1. Safely create the first string
  NS_TRY(err, ns_string_new(&str1, "Hello, ")) {
    ns_println(NS_COLOR_GREEN "Created str1 successfully." NS_COLOR_RESET);
  } NS_EXCEPT(err, NS_ERROR_ANY) {
    ns_println(NS_COLOR_RED "Failed to create str1." NS_COLOR_RESET);
    return 1;
  }

  // 2. Safely create the second string (Long enough to force a heap allocation!)
  NS_TRY(err, ns_string_new(&str2, "World! This is a safe C string library.")) {
    ns_println(NS_COLOR_GREEN "Created str2 successfully." NS_COLOR_RESET);
  } NS_EXCEPT(err, NS_ERROR_ANY) {
    ns_println(NS_COLOR_RED "Failed to create str2." NS_COLOR_RESET);
    return 1;
  }

  // 3. Safely concatenate them together
  ns_println("\nAttempting to concatenate str1 and str2...");

  NS_TRY(err, ns_string_concat(&result, str1, str2)) {

    ns_print(NS_COLOR_GREEN NS_COLOR_BOLD "Success! Result: " NS_COLOR_RESET);

    // Print the final string using your safe I/O macro!
    ns_println_ns_string(result);

  } NS_EXCEPT(err, NS_ERROR_STRING_ALLOC) {

    ns_println(NS_COLOR_RED "Failed: Out of heap memory!" NS_COLOR_RESET);

  } NS_EXCEPT(err, NS_ERROR_ANY) {

    ns_print(NS_COLOR_RED "An unknown error occurred: ");
    ns_println(ns_error_message(err));
    ns_print(NS_COLOR_RESET);

  }

  // 4. Test the NULL pointer crash-proofing
  ns_println(NS_COLOR_CYAN NS_COLOR_BOLD "\n---- Testing NULL Pointer Safety ----" NS_COLOR_RESET);

  ns_string* bad_dest = 0; // NULL pointer

  NS_TRY(err, ns_string_concat(bad_dest, str1, str2)) {
    ns_println(NS_COLOR_RED "CRITICAL FAILURE: This should not print!" NS_COLOR_RESET);
  } NS_EXCEPT(err, NS_ERROR_ANY) {
    ns_print(NS_COLOR_YELLOW "Safely caught the NULL pointer! Error Message: ");
    ns_println(ns_error_message(err));
    ns_print(NS_COLOR_RESET);
  }

  // 5. Clean up the memory to prevent leaks
  ns_string_free(&str1);
  ns_string_free(&str2);
  ns_string_free(&result);

  ns_println(NS_COLOR_GREEN "\nMemory successfully freed. Exiting safely." NS_COLOR_RESET);

  return 0;
}
