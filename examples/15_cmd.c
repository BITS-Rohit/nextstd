#include "../include/ns.h"
#include "../include/ns_cmd.h"

int main(void)
{
  ns_println("NextStd Command Execution Demo");
  ns_println("Test 1: Standard C String Execution");
  {
    // Zero initialize to ensure safe auto-cleanup
    ns_autocmd ns_cmd_output out1 = {0};

    ns_println("Executing uname -a");
    ns_cmd_run("uname -a", &out1);

    ns_println("Exit Code: {}", out1.exit_code);
    ns_println("Stdout: {}", out1.stdout_data);
  }

  ns_println("\nTest 2: Dynamic ns_string execution");
  {
    ns_autocmd ns_cmd_output out2 = {0};

    ns_string my_cmd;
    ns_string_new(&my_cmd, "echo 'Command execution is now memory safe'");

    ns_println("Executing echo 'Command execution is now memory safe'");
    ns_cmd_run(my_cmd, &out2);

    ns_println("Exit Code: {}", out2.exit_code);
    ns_println("Stdout: {}", out2.stdout_data);

    // Free the input string 
    ns_string_free(&my_cmd);
  }

  ns_println("\nTest 3: Stderr");
  {
    ns_autocmd ns_cmd_output out_err = {0};

    ns_println("Executing: cat non_existent_file.txt");
    ns_cmd_run("cat non_existent_file", &out_err);

    ns_println("Exit Code: {}", out_err.exit_code);
    ns_println("Stderr: {}", out_err.stderr_data);
  }

  return 0;
}
