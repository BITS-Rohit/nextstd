#ifndef NS_CMD_H
#define NS_CMD_H

#include "ns_string.h"
#include "ns_error.h"

#ifdef __cplusplus 
extern "C" {
#endif

  typedef struct ns_cmd_output {
    ns_string stdout_data;
    ns_string stderr_data;
    int exit_code;
  } ns_cmd_output;

  // For both string types
  ns_error_t ns_cmd_run_cstr(const char* command, ns_cmd_output* output);
  ns_error_t ns_cmd_run_ns(ns_string command, ns_cmd_output* output);

  // Destructor function to auto free memory of the strings in the struct 
  void ns_cmd_output_free(ns_cmd_output* output);

  // Cleanup function 
  static inline void ns_cmd_cleanup_helper(ns_cmd_output* ptr) {
    if (ptr) {
      ns_cmd_output_free(ptr);
    }
  }

  // Routes the command based on string type 
#define ns_cmd_run(cmd, out) _Generic((cmd), \
    char*: ns_cmd_run_cstr, \
    const char*: ns_cmd_run_cstr, \
    ns_string: ns_cmd_run_ns \
)(cmd, out)

  // Auto free macro 
#define ns_autocmd __attribute__((cleanup(ns_cmd_cleanup_helper)))

#ifdef __cplusplus
}
#endif

#endif // !NS_CMD_H
