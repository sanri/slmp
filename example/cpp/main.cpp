#include <iostream>
#include "slmp.h"

using namespace std;

void printdata(uint16_t *data, int n) {
  for (int i = 0; i < n; i++) {
    std::cout << data[i] << ", ";
  }
  std::cout << std::endl;
}

int main(int argc, char*argv[]) {
  cout << "hello" << endl;
  Slmp handle = slmp_connect("192.168.10.61", 5000);
  if (handle == nullptr) {
    cout << "slmp connect fault" << endl;
    return 0;
  }
  cout << "slmp connect successful" << endl;

  int32_t result = 0;
  uint16_t r_data[10] = {0};

  result = slmp_read_words(handle, 1, 1, 10, r_data);
  std::cout << "read words = " << result << std::endl;
  if (result != 0)
    return 0;
  printdata(r_data, 10);

  uint16_t w_data[10] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 0};
  result = slmp_write_words(handle, 1, 1, 10, w_data);
  std::cout << "write words = " << result << std::endl;
  if (result != 0)
    return 0;

  memset(r_data, 0, 10);
  result = slmp_read_words(handle, 1, 1, 10, r_data);
  std::cout << "read words = " << result << std::endl;
  if (result != 0)
    return 0;
  printdata(r_data, 10);

  slmp_shutdown(handle);
  cout << "slmp shutdown" << endl;
  return 0;
}
