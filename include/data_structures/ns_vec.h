#ifndef NS_VEC_H
#define NS_VEC_H

#include <stddef.h>
#include <stdbool.h>

#include "../ns_error.h"

#ifdef __cplusplus
extern "C" {
#endif

  typedef struct ns_vec {
    unsigned char *data;
    size_t len;
    size_t capacity;
    size_t element_size;
  } ns_vec;

  ns_error_t ns_vec_new(ns_vec* dest, size_t element_size);
  ns_error_t ns_vec_grow(ns_vec* v);

  void ns_vec_free(ns_vec* v);

  // Pushes a value into a vector 
  // Returns NS_ERROR_SUCCESS or an error if heap allocation fails
#define ns_vec_push(v_ptr, type, val) \
    ( \
      ((v_ptr)->len >= (v_ptr)->capacity && ns_vec_grow(v_ptr) != 0 /* NS_ERROR_SUCCESS */) \
      ? 1 /* NS_ERROR_ANY */ \
      : ((((type*)((v_ptr)->data))[(v_ptr)->len++] = (val)), 0 /* NS_ERROR_SUCCESS */) \
    )

  // Retrieves a value from the vector with strict bounds checking.
  // Returns NS_ERROR_SUCCESS, or an error if the index is out of bounds.
  #define ns_vec_get(v_ptr, type, index, out_ptr) \
    ( \
      ((index) < (v_ptr)->len) \
      ? (*(out_ptr) = ((type*)((v_ptr)->data))[index], 0 /* NS_ERROR_SUCCESS */) \
      : 1 /* NS_ERROR_ANY or NS_ERROR_OUT_OF_BOUNDS */ \
    )
#ifdef __cplusplus

}

#endif // __cplusplus

#endif // !NS_VEC_H
