#ifndef NS_READ_H
#define NS_READ_H

#include <stddef.h>
#include "ns_error.h"
#include "ns_string.h"

// Check if we are in C++ 
#ifdef __cplusplus
extern "C" {
#endif

  // ----Reading functions------ 
  ns_error_t ns_read_int(int* ptr);
  ns_error_t ns_read_float(float* ptr);
  ns_error_t ns_read_double(double* ptr);
  ns_error_t ns_read_bool(_Bool* ptr);
  ns_error_t ns_read_ns_string(ns_string* ptr);

  // Generic Read macro 
#define ns_read(x) _Generic((x), \
    int*: ns_read_int, \
    float*: ns_print_float, \
    double*: ns_read_double, \
    _Bool*: ns_read_bool, \
    ns_string*: ns_read_ns_string \
)(x)

#ifdef __cplusplus
}
#endif // !__cplusplus

#endif // !NS_READ_H
