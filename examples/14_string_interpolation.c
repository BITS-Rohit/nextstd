#include "../include/ns.h"
#include "../include/ns_color.h"

int main(void)
{
  ns_println("NextStd String Interpolation");

  // Mixed Types
  int rewatch_count = 5;
  const char* anime = "Frieren: beyond Journey's end";
  ns_println("My Favorite anime is {} and I have watched it {} times", anime, rewatch_count);

  float temp = 45.2f;
  _Bool is_active = 1;

  ns_println("System Active: {}, CPU Temperature: {}", is_active, temp);

  ns_error_t err;
  ns_string os_name;

  NS_TRY(err, ns_string_new(&os_name, "Endeavour OS")) {
    ns_println("I am currently running {} on my machine", os_name);
    ns_string_free(&os_name);
  } NS_EXCEPT(err, NS_ERROR_ANY) {
    ns_println(ns_error_message(err));
  }

  // safety Test (This normally causes a segfault in C)
  ns_println("\n----Safety Test-----");
  ns_println("Expected 3 vars, gave 2: {}, {}, {}", 100, 200);

  return 0;
}
