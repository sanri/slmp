#pragma once


#ifdef _WIN32
#define API_PREFIX __declspec(dllimport)
#else
#define API_PREFIX
#endif


#ifdef __cplusplus
#include <cstdint>
extern "C" {
#else
#include <stdint.h>
#endif

typedef void* Slmp;

//连接失败返回 null
API_PREFIX Slmp slmp_connect(const char* ip,uint16_t port);

//调用此函数断开连接后，还会释放内部资源。
//所以重复调用此函数会使程序崩溃
API_PREFIX void slmp_shutdown(Slmp slmp);

//批量读取字软元件
//dev  1:保持寄存器D, 2:文件寄存器R
//执行成功返回 0
API_PREFIX int32_t slmp_read_words(Slmp slmp,uint32_t head_number,uint16_t dev,uint16_t number,uint16_t* data);

//批量读取位软元件
//dev  1: 内部继电器M, 2: 输入继电器X, 3: 输出继电器Y
//执行成功返回 0
API_PREFIX int32_t slmp_read_bits(Slmp slmp,uint32_t head_number,uint16_t dev,uint16_t number, uint8_t* data);

//批量写入字软元件
//dev  1:保持寄存器D, 2:文件寄存器R
//执行成功返回 0
API_PREFIX int32_t slmp_write_words(Slmp slmp,uint32_t head_number,uint16_t dev,uint16_t number,const uint16_t* data);

//批量写入位软元件
//dev  1: 内部继电器M, 2: 输入继电器X, 3: 输出继电器Y
//执行成功返回 0
API_PREFIX int32_t slmp_write_bits(Slmp slmp,uint32_t head_number,uint16_t dev,uint16_t number,const uint8_t* data);

#ifdef __cplusplus
}
#endif
