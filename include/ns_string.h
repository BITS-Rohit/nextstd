#ifndef NS_STRING_H
#define NS_STRING_H

#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

  // Heap allocation details
  typedef struct ns_string_heap{
    char *ptr;
    size_t capacity;
  } ns_string_heap;

  // Union : 24 bytes of inline chars OR the heap struct
  typedef union ns_string_data{
    char inline_data[24];
    ns_string_heap heap;
  } ns_string_data;

  // Final string struct
  typedef struct ns_string{
    size_t len;
    bool is_heap;
    ns_string_data data;
  } ns_string;

  // Core functions
  ns_string ns_string_new(const char* c_str);
  void ns_string_free(ns_string* s);

#ifdef __cplusplus

}

#endif // !__cplusplus

#endif // !NS_STRING_H
