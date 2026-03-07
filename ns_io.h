#ifndef NS_IO_H
#define NS_IO_H

// Check if we are in C++ 
#ifdef __cplusplus
extern "C" {
#endif

  // -----Printing functions------
  // No Newline
  // int in C is equivalent to i32 in Rust
  void ns_print_int(int val);

  // Float in C is equivalent to f32 in Rust
  void ns_print_float(float val);

  // Double in C is equivalent to f64 in Rust
  void ns_print_double(double val);

  // Printing Strings
  void ns_print_string(const char* val);

  // Newline
  // int in C is equivalent to i32 in Rust
  void ns_println_int(int val);

  // Float in C is equivalent to f32 in Rust
  void ns_println_float(float val);

  // Double in C is equivalent to f64 in Rust
  void ns_println_double(double val);

  // Printing Strings
  void ns_println_string(const char* val);

  // ----Reaing functions------ 
  void ns_read_int(int* ptr);
  void ns_read_float(float* ptr);
  void ns_read_double(double* ptr);

  // Generic Macro 
  // No Newline
#define ns_print(x) _Generic((x), \
	int: ns_print_int, \
	float: ns_print_float, \
	double: ns_print_double, \
	char*:  ns_print_string, \
  const char*: ns_print_string \
)(x)


  // With newline
#define ns_println(x) _Generic((x), \
	int: ns_println_int, \
	float: ns_println_float, \
	double: ns_println_double, \
	char*:  ns_println_string, \
  const char*: ns_println_string \
)(x)

  // Generic Read macro 
#define ns_read(x) _Generic((x), \
    int*: ns_read_int, \
    float*: ns_print_float, \
    double*: ns_read_double \
)(x)

#ifdef __cplusplus
}
#endif // !__cplusplus

#endif // !NS_IO_H
