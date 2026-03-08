#ifndef NS_PRINT_H
#define NS_PRINT_H

#include "ns_string.h" // Needs to know about ns_string

#ifdef __cplusplus
extern "C" {
#endif

  // -----Printing functions------
  // No Newline
  void ns_print_int(int val);
  void ns_print_float(float val);
  void ns_print_double(double val);
  void ns_print_string(const char* val);
  void ns_print_ns_string(ns_string val); // <-- NEW DECLARATION

  // Newline
  void ns_println_int(int val);
  void ns_println_float(float val);
  void ns_println_double(double val);
  void ns_println_string(const char* val);
  void ns_println_ns_string(ns_string val); // <-- NEW DECLARATION

  // Generic Macro (No Newline)
#define ns_print(x) _Generic((x), \
    int: ns_print_int, \
    float: ns_print_float, \
    double: ns_print_double, \
    char*: ns_print_string, \
    const char*: ns_print_string, \
    ns_string: ns_print_ns_string \
)(x)

  // Generic Macro (With newline)
#define ns_println(x) _Generic((x), \
    int: ns_println_int, \
    float: ns_println_float, \
    double: ns_println_double, \
    char*: ns_println_string, \
    const char*: ns_println_string, \
    ns_string: ns_println_ns_string \
)(x)

#ifdef __cplusplus
}
#endif // !__cplusplus

#endif // !NS_PRINT_H
