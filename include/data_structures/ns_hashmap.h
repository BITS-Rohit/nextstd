#ifndef NS_HASHMAP_H
#define NS_HASHMAP_H

#include <stddef.h>
#include "../ns_error.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    void* internal;
    size_t key_size;
    size_t val_size;
    size_t len;
} ns_map;

// FFI declarations from Rust
extern ns_error_t ns_map_new(ns_map* dest, size_t key_size, size_t val_size);
extern ns_error_t ns_map_insert(ns_map* m, const void* key_ptr, const void* val_ptr);
extern ns_error_t ns_map_get(const ns_map* m, const void* key_ptr, void* out_val_ptr);
extern void ns_map_free(ns_map* m);

/**
 * @brief Insert a key-value pair into the map.
 * Usage: ns_map_put(&my_map, int, my_key, float, my_val);
 */
#define ns_map_put(m, key_type, key, val_type, val) ({ \
    key_type _k = (key); \
    val_type _v = (val); \
    ns_map_insert((m), &_k, &_v); \
})

/**
 * @brief Retrieve a value from the map by key.
 * Usage: float result; ns_map_at(&my_map, int, my_key, &result);
 */
#define ns_map_at(m, key_type, key, out_ptr) ({ \
    key_type _k = (key); \
    ns_map_get((m), &_k, (out_ptr)); \
})

#ifdef __cplusplus
}
#endif

#endif // NS_HASHMAP_H
