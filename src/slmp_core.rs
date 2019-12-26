use std::net::{IpAddr,Ipv4Addr};
use std::vec;
use std::path::Prefix::DeviceNS;

//字软元件
enum DeviceWord {
  D, //数据寄存器 D
}

//位软元件
enum DeviceBit{
  X, //输入继电器 X
  Y, //输出继电器 Y
  M, //内部继电器 M
}


trait Request{
  //序列化
  fn serialize(&self)->Vec<u8>;
}

trait Response{
//反序列化 Deserialization
}

//批量读 字
fn read_words(device:DeviceWord, head_number:u32, number:u16){

}

//批量读 位
fn read_bits(device:DeviceBit,head_number:u32,number:u16){

}

const requset:[u8;2]=[0x50,0x00];
const response:[u8;2]=[0xD0,0x00];

//软元件代码
enum DeviceCode{
  X = 0x9c, //输入继电器 X
  Y = 0x9d, //输出继电器 Y
  M = 0x90, //内部继电器 M
  D = 0xA8, //数据寄存器 D
}

struct Destination{
  network:u8,           //网络编号
  station:u8,           //站号
  module:u16,           //模块IO编号
  multidrop_station:u8, //多点站号
}

impl Destination {
  fn new()->Destination {
    Destination {
      network: 0x00,
      station: 0xff,
      module: 0x03ff,
      multidrop_station: 0x00
    }
  }

  fn serialize(&self) -> [u8; 5] {
    //网络编号(1) + 站号(1) + 模块编号(2) + 多点站号(1) = 5 字节
    let mut out: [u8; 5] = [0; 5];
    out[0] = self.network;
    out[1] = self.station;
    let t: [u8; 2] = self.module.to_le_bytes();
    out[2] = t[0];
    out[3] = t[1];
    out[4] = self.multidrop_station;
    return out;
  }
}

//批量读请求(字软元件)
struct ReqReadWords{
  des:Destination,
  device:DeviceWord,
  head_number:u32,
  number:u16,
}

impl Request for ReqReadWords {
  fn serialize(&self) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(30);
    for &i in &requset{
      out.push(i);
    }
    let d = Destination::new().serialize();
    for &i in &d{
      out.push(d[i]);
    }
    for i in 0..4{
      out.push(0);
    }
    //指令
    out.push(0x01);
    out.push(0x04);
    //子指令
    out.push(0x00);
    out.push(0x00);
    //起始软元件编号
    let h = self.head_number.to_le_bytes();
    for i in 0..3{
      out.push(h[i]);
    }
    //软元件代码
    out.push(DeviceCode::D as u8);


    out
  }
}



//批量读响应
struct ResReadWords{
  des:Destination,
}


//批量读请求(位软元件)
struct ReqReadBits{
  des:Destination,
  device:DeviceBit,
  head_number:u32,
  number:u16,
}




