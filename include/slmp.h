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

//此函数在断开连接后，还会释放内部资源。
//所以重复调用此函数会使程序崩溃
API_PREFIX void slmp_shutdown(Slmp slmp);

//批量读取D软元件
API_PREFIX int32_t slmp_read_words(Slmp slmp,uint32_t head_number,uint16_t number,uint16_t* data);

//批量写入D软元件
API_PREFIX int32_t slmp_write_words(Slmp slmp,uint32_t head_number,uint16_t number,uint16_t* data);

#ifdef __cplusplus
}
#endif
