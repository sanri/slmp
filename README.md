# SLMP
三菱SLMP通信协议的Rust实现

```c++
#include "slmp.h"
#include <iostream>

void printdata(uint16_t *data, int n) {
  for (int i = 0; i < n; i++) {
    std::cout << data[i] << ", ";
  }
  std::cout << std::endl;
}

int main() {
  Slmp slmp = slmp_connect("192.168.10.250", 2025);
  if (slmp == nullptr)
    return 0;
  int32_t r;
  uint16_t r_data[10] = {0};

  r = slmp_read_words(slmp, 1, 10, r_data);
  std::cout << "read words = " << r << std::endl;
  if (r != 0)
    return 0;
  printdata(r_data, 10);

  uint16_t w_data[10] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 0};
  r = slmp_write_words(slmp, 1, 10, w_data);
  std::cout << "write words = " << r << std::endl;
  if (r != 0)
    return 0;

  memset(r_data, 0, 10);
  r = slmp_read_words(slmp, 1, 10, r_data);
  std::cout << "read words = " << r << std::endl;
  if (r != 0)
    return 0;
  printdata(r_data, 10);

  slmp_shutdown(slmp);
  return 0;
}
```