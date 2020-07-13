#include <iostream>
#include "slmp.h"

using namespace std;

void print_data_words(uint16_t *data, int n) {
  for (int i = 0; i < n; i++) {
    cout << data[i] << ", ";
  }
  cout << endl;
}

void print_data_bits(uint8_t *data, int n) {
  for (int i = 0; i < n; i++) {
    string b;
    if (data[i] == 0) {
      b = "false";
    } else {
      b = "true";
    }
    cout << b << ", ";
  }
  cout << endl;
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

  // 字软元件读写 ---------------------------------------------
  uint16_t w_data[10] = {0};

  result = slmp_read_words(handle, 1, 1, 10, w_data);
  cout << "read words = " << result << endl;
  if (result != 0)
    return 0;
  print_data_words(w_data, 10);

  uint16_t ww_data[10] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 0};
  result = slmp_write_words(handle, 1, 1, 10, ww_data);
  cout << "write words = " << result << endl;
  if (result != 0)
    return 0;

  memset(w_data, 0, 10);
  result = slmp_read_words(handle, 1, 1, 10, w_data);
  cout << "read words = " << result << endl;
  if (result != 0)
    return 0;
  print_data_words(w_data, 10);
  cout << endl;

  // 位软元件读写 ---------------------------------------------
  uint8_t b_data[10] = {0};

  result = slmp_read_bits(handle, 3, 1, 10, b_data);
  cout << "read bits = " << result << endl;
  if (result != 0)
    return 0;
  print_data_bits(b_data, 10);

  uint8_t bb_data[10] = {1, 0, 1, 0, 0, 1, 0, 0, 0, 1};
  result = slmp_write_bits(handle, 3, 1, 10, bb_data);
  cout << "write bits = " << result << endl;
  if (result != 0)
    return 0;

  memset(b_data, 0, 10);
  result = slmp_read_bits(handle, 3, 1, 10, b_data);
  cout << "read bits = " << result << endl;
  if (result != 0)
    return 0;
  print_data_bits(b_data, 10);
  cout << endl;

  // -------------------------------------------------------
  slmp_shutdown(handle);
  cout << "slmp shutdown" << endl;
  return 0;
}
