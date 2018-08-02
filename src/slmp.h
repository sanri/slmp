#pragma once

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus


#include <stdint.h>
#include <stdbool.h>

#include "kdq.h"


KDQ_INIT(char);

enum slmp_frame_type
{
    slmp_frame_read_command,
    slmp_frame_read_response,
    slmp_frame_read_error,
    slmp_frame_write_command,
    slmp_frame_write_response,
    slmp_frame_write_error
};

enum slmp_command
{
    slmp_command_read,                  //批量读取
    slmp_command_write,                 //批量写入
    slmp_command_read_random,           //随机读取
    slmp_command_write_random,          //随机写入
    slmp_command_read_block,            //批量读取多个块
    slmp_command_write_block,           //批量写入多个块
    slmp_command_remote_run,            //远程RUN
    slmp_command_remote_stop,           //远程STOP
    slmp_command_remote_pause,          //远程PAUSE
    slmp_command_remote_latch_clear,    //远程锁存清除
    slmp_command_remote_reset,          //远程复位
    slmp_command_read_type_name,        //读取CPU型号
    slmp_command_self_test,             //通信测试
    slmp_command_clear_error,           //清除错误
    slmp_command_password_lock,         //锁定
    slmp_command_password_unlock,       //解锁
};

typedef struct  slmp_data_read_command
{

};

typedef struct slmp_data_read_response
{

};

typedef struct slmp_data_write_command
{

};

typedef struct slmp_frame
{
    enum slmp_frame_type type;
    uint8_t network_number;
    uint8_t station_number;
    uint16_t module_io_number;
    uint8_t multi_drop_station_number;

};

typedef struct slmp
{
    kdq_char_t buffer;
}slmp_t;

//将数据填入缓冲区,返回当前缓冲区已有数据的大小
int32_t slmp_input(slmp_t *sl,const char* input);





#ifdef __cplusplus
}
#endif // __cplusplus
