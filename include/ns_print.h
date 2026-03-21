#ifndef NS_PRINT_H
#define NS_PRINT_H

#include <stddef.h>
#include "ns_string.h"

#ifdef __cplusplus
extern "C" {
#endif

// Formatting & Interpolation Types

// 1. The Tag: Identifies what type is stored in the union
typedef enum ns_type_tag {
    NS_TYPE_INT,
    NS_TYPE_FLOAT,
    NS_TYPE_DOUBLE,
    NS_TYPE_BOOL,
    NS_TYPE_SIZE_T,
    NS_TYPE_CSTR,
    NS_TYPE_NS_STRING
} ns_type_tag;

// 2. The Data: A union that is exactly as large as its largest member
typedef union ns_any_data {
    int v_int;
    float v_float;
    double v_double;
    _Bool v_bool;
    size_t v_size_t;
    const char* v_cstr;
    ns_string v_ns_string;
} ns_any_data;

// 3. The Package: A single struct we can pass to Rust safely
typedef struct ns_any_t {
    ns_type_tag tag;
    ns_any_data data;
} ns_any_t;

// 4. Constructor Functions (Shields from strict _Generic type-checking and nested designator warnings)
static inline ns_any_t ns_any_from_int(int v) { ns_any_t a; a.tag = NS_TYPE_INT; a.data.v_int = v; return a; }
static inline ns_any_t ns_any_from_float(float v) { ns_any_t a; a.tag = NS_TYPE_FLOAT; a.data.v_float = v; return a; }
static inline ns_any_t ns_any_from_double(double v) { ns_any_t a; a.tag = NS_TYPE_DOUBLE; a.data.v_double = v; return a; }
static inline ns_any_t ns_any_from_bool(_Bool v) { ns_any_t a; a.tag = NS_TYPE_BOOL; a.data.v_bool = v; return a; }
static inline ns_any_t ns_any_from_size_t(size_t v) { ns_any_t a; a.tag = NS_TYPE_SIZE_T; a.data.v_size_t = v; return a; }
static inline ns_any_t ns_any_from_cstr(const char* v) { ns_any_t a; a.tag = NS_TYPE_CSTR; a.data.v_cstr = v; return a; }
static inline ns_any_t ns_any_from_ns_string(ns_string v) { ns_any_t a; a.tag = NS_TYPE_NS_STRING; a.data.v_ns_string = v; return a; }

// 5. The Magic Converter: Selects the correct constructor function
#define ns_to_any(x) _Generic((x), \
    int: ns_any_from_int, \
    float: ns_any_from_float, \
    double: ns_any_from_double, \
    _Bool: ns_any_from_bool, \
    size_t: ns_any_from_size_t, \
    char*: ns_any_from_cstr, \
    const char*: ns_any_from_cstr, \
    ns_string: ns_any_from_ns_string \
)(x)

// Rust FFI Functions

// Formatted printing (Our new Rust functions)
void ns_print_fmt_c(const char* fmt, ns_any_t* args, size_t num_args);
void ns_println_fmt_c(const char* fmt, ns_any_t* args, size_t num_args);

// Single-variable printing (No Newline)
void ns_print_int(int val);
void ns_print_float(float val);
void ns_print_double(double val);
void ns_print_bool(_Bool val);
void ns_print_size_t(size_t val);
void ns_print_string(const char* val);
void ns_print_ns_string(ns_string val); 

// Single-variable printing (Newline)
void ns_println_int(int val);
void ns_println_float(float val);
void ns_println_double(double val);
void ns_println_bool(_Bool val);
void ns_println_size_t(size_t val);
void ns_println_string(const char* val);
void ns_println_ns_string(ns_string val);

// The Macro Magic (Overloading by Argument Count)

// 1. Single argument dispatcher (The original generic macros)
#define ns_print_1(x) _Generic((x), \
    int: ns_print_int, \
    float: ns_print_float, \
    double: ns_print_double, \
    _Bool: ns_print_bool, \
    size_t: ns_print_size_t, \
    char*: ns_print_string, \
    const char*: ns_print_string, \
    ns_string: ns_print_ns_string \
)(x)

#define ns_println_1(x) _Generic((x), \
    int: ns_println_int, \
    float: ns_println_float, \
    double: ns_println_double, \
    _Bool: ns_println_bool, \
    size_t: ns_println_size_t, \
    char*: ns_println_string, \
    const char*: ns_println_string, \
    ns_string: ns_println_ns_string \
)(x)

// 2. Formatted argument dispatchers (Supports up to 8 variables formatted at once)
// ns_print (No newline)
#define ns_print_2(fmt, a) ns_print_fmt_c(fmt, (ns_any_t[]){ns_to_any(a)}, 1)
#define ns_print_3(fmt, a, b) ns_print_fmt_c(fmt, (ns_any_t[]){ns_to_any(a), ns_to_any(b)}, 2)
#define ns_print_4(fmt, a, b, c) ns_print_fmt_c(fmt, (ns_any_t[]){ns_to_any(a), ns_to_any(b), ns_to_any(c)}, 3)
#define ns_print_5(fmt, a, b, c, d) ns_print_fmt_c(fmt, (ns_any_t[]){ns_to_any(a), ns_to_any(b), ns_to_any(c), ns_to_any(d)}, 4)
#define ns_print_6(fmt, a, b, c, d, e) ns_print_fmt_c(fmt, (ns_any_t[]){ns_to_any(a), ns_to_any(b), ns_to_any(c), ns_to_any(d), ns_to_any(e)}, 5)
#define ns_print_7(fmt, a, b, c, d, e, f) ns_print_fmt_c(fmt, (ns_any_t[]){ns_to_any(a), ns_to_any(b), ns_to_any(c), ns_to_any(d), ns_to_any(e), ns_to_any(f)}, 6)
#define ns_print_8(fmt, a, b, c, d, e, f, g) ns_print_fmt_c(fmt, (ns_any_t[]){ns_to_any(a), ns_to_any(b), ns_to_any(c), ns_to_any(d), ns_to_any(e), ns_to_any(f), ns_to_any(g)}, 7)
#define ns_print_9(fmt, a, b, c, d, e, f, g, h) ns_print_fmt_c(fmt, (ns_any_t[]){ns_to_any(a), ns_to_any(b), ns_to_any(c), ns_to_any(d), ns_to_any(e), ns_to_any(f), ns_to_any(g), ns_to_any(h)}, 8)

// ns_println (With newline)
#define ns_println_2(fmt, a) ns_println_fmt_c(fmt, (ns_any_t[]){ns_to_any(a)}, 1)
#define ns_println_3(fmt, a, b) ns_println_fmt_c(fmt, (ns_any_t[]){ns_to_any(a), ns_to_any(b)}, 2)
#define ns_println_4(fmt, a, b, c) ns_println_fmt_c(fmt, (ns_any_t[]){ns_to_any(a), ns_to_any(b), ns_to_any(c)}, 3)
#define ns_println_5(fmt, a, b, c, d) ns_println_fmt_c(fmt, (ns_any_t[]){ns_to_any(a), ns_to_any(b), ns_to_any(c), ns_to_any(d)}, 4)
#define ns_println_6(fmt, a, b, c, d, e) ns_println_fmt_c(fmt, (ns_any_t[]){ns_to_any(a), ns_to_any(b), ns_to_any(c), ns_to_any(d), ns_to_any(e)}, 5)
#define ns_println_7(fmt, a, b, c, d, e, f) ns_println_fmt_c(fmt, (ns_any_t[]){ns_to_any(a), ns_to_any(b), ns_to_any(c), ns_to_any(d), ns_to_any(e), ns_to_any(f)}, 6)
#define ns_println_8(fmt, a, b, c, d, e, f, g) ns_println_fmt_c(fmt, (ns_any_t[]){ns_to_any(a), ns_to_any(b), ns_to_any(c), ns_to_any(d), ns_to_any(e), ns_to_any(f), ns_to_any(g)}, 7)
#define ns_println_9(fmt, a, b, c, d, e, f, g, h) ns_println_fmt_c(fmt, (ns_any_t[]){ns_to_any(a), ns_to_any(b), ns_to_any(c), ns_to_any(d), ns_to_any(e), ns_to_any(f), ns_to_any(g), ns_to_any(h)}, 8)

// 3. The Router Macros
#define NS_GET_MACRO(_1, _2, _3, _4, _5, _6, _7, _8, _9, NAME, ...) NAME

#define ns_print(...) \
    NS_GET_MACRO(__VA_ARGS__, ns_print_9, ns_print_8, ns_print_7, ns_print_6, ns_print_5, ns_print_4, ns_print_3, ns_print_2, ns_print_1)(__VA_ARGS__)

#define ns_println(...) \
    NS_GET_MACRO(__VA_ARGS__, ns_println_9, ns_println_8, ns_println_7, ns_println_6, ns_println_5, ns_println_4, ns_println_3, ns_println_2, ns_println_1)(__VA_ARGS__)

#ifdef __cplusplus
}
#endif // !__cplusplus

#endif // !NS_PRINT_H
