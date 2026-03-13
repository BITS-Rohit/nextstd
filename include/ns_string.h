#ifndef NS_STRING_H
#define NS_STRING_H

#include <stddef.h>
#include <stdbool.h>
#include "ns_error.h" // <-- Import your new error types!

#ifdef __cplusplus
extern "C" {
#endif

  // Heap allocation details
  typedef struct ns_string_heap {
    char *ptr;
    size_t capacity;
  } ns_string_heap;

  // Union : 24 bytes of inline chars OR the heap struct
  typedef union ns_string_data {
    char inline_data[24];
    ns_string_heap heap;
  } ns_string_data;

  // Final string struct
  typedef struct ns_string {
    size_t len;
    bool is_heap;
    ns_string_data data;
  } ns_string;

  // ==========================================
  // Core String Functions
  // ==========================================

  // Safely creates a new string, returning an error if heap allocation fails.
  // The result is written directly into the `dest` pointer.
  ns_error_t ns_string_new(ns_string* dest, const char* c_str);

  // Safely concatenates two strings, returning an error if heap allocation fails.
  // The result is written directly into the `dest` pointer.
  ns_error_t ns_string_concat(ns_string* dest, ns_string s1, ns_string s2);

  // Safely frees heap memory if the string used it. 
  // Safe to call on inline strings (it just zeroes them out).
  void ns_string_free(ns_string* s);

#ifdef __cplusplus
}
#endif // !__cplusplus

#endif // !NS_STRING_H
