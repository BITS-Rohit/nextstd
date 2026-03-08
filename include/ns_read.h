#ifndef NS_READ_H
#define NS_READ_H



// Check if we are in C++ 
#ifdef __cplusplus
extern "C" {
#endif

  // ----Reaing functions------ 
  void ns_read_int(int* ptr);
  void ns_read_float(float* ptr);
  void ns_read_double(double* ptr);

  // Generic Read macro 
#define ns_read(x) _Generic((x), \
    int*: ns_read_int, \
    float*: ns_print_float, \
    double*: ns_read_double \
)(x)

#ifdef __cplusplus
}
#endif // !__cplusplus

#endif // !NS_READ_H
