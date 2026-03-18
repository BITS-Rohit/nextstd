#include "../include/ns.h"
#include "../include/ns_data.h"

int main() {
  ns_error_t err;
  ns_map inventory;

  // 1. Initialize HashMap: Keys are strings, Values are integers
  NS_TRY(err, ns_map_new(&inventory, sizeof(ns_string), sizeof(int))) {
    ns_println("Inventory initialized.");
  } NS_EXCEPT(err, NS_ERROR_ANY) {
    return 1;
  }

  // 2. Create string keys
  ns_string item1, item2, item3;
  ns_string_new(&item1, "Apples");
  ns_string_new(&item2, "Bananas");
  ns_string_new(&item3, "Cherries");

  // 3. Insert items into the map
  ns_map_put(&inventory, ns_string, item1, int, 50);
  ns_map_put(&inventory, ns_string, item2, int, 120);

  // 4. Safely retrieve an existing item
  int stock = 0;
  NS_TRY(err, (ns_map_at(&inventory, ns_string, item1, &stock))) {
    ns_print("Apples in stock: ");
    ns_println(stock);
  } NS_EXCEPT(err, NS_ERROR_ANY) {
    ns_println("Apples not found!");
  }

  // 5. Attempt to retrieve a non-existent item (Safe failure)
  NS_TRY(err, (ns_map_at(&inventory, ns_string, item3, &stock))) {
    ns_println("Cherries found!");
  } NS_EXCEPT(err, NS_ERROR_ANY) {
    ns_println("Cherries not found in inventory. Safe failure!");
  }

  // 6. Cleanup memory
  ns_string_free(&item1);
  ns_string_free(&item2);
  ns_string_free(&item3);
  ns_map_free(&inventory);

  return 0;
}
