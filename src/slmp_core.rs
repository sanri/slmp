use std::net::{IpAddr,Ipv4Addr};
use std::vec;
use std::path::Prefix::DeviceNS;

//字软元件
pub(crate) enum DeviceWord {
  D, //数据寄存器 D
}

//位软元件
pub(crate) enum DeviceBit{
  X, //输入继电器 X
  Y, //输出继电器 Y
  M, //内部继电器 M
}


//request 请求
pub(crate) trait Req{
  //序列化
  fn serialize(&self)->Vec<u8>;
}

//response 响应
pub(crate) trait Res {
  //反序列化 Deserialization
  fn deserialization(&mut self, data: &[u8]) -> std::result::Result<(), ()>;
}

//批量读 字
fn read_words(device:DeviceWord, head_number:u32, number:u16){

}

//批量读 位
fn read_bits(device:DeviceBit,head_number:u32,number:u16){

}

const REQUSET:[u8;2]=[0x50,0x00];
const RESPONSE:[u8;2]=[0xD0,0x00];

//软元件代码
enum DeviceCode{
  X = 0x9c, //输入继电器 X
  Y = 0x9d, //输出继电器 Y
  M = 0x90, //内部继电器 M
  D = 0xA8, //数据寄存器 D
}

pub(crate) struct Destination{
  network:u8,           //网络编号
  station:u8,           //站号
  module:u16,           //模块IO编号
  multidrop_station:u8, //多点站号
}

impl Destination {
  pub(crate) fn new() -> Destination {
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

  fn deserialization(&mut self,data: &[u8]) ->Result<(),()> {
    //网络编号(1) + 站号(1) + 模块编号(2) + 多点站号(1) = 5 字节
    if data.len() < 5 {
      return Err(());
    }
    self.network = data[0];
    self.station = data[1];
    self.module = u16::from_le_bytes([data[2], data[3]]);
    self.multidrop_station = data[4];
    return Ok(());
  }
}

//批量读请求(字软元件)
pub(crate) struct ReqReadWords{
  des:Destination,
  device:DeviceWord,
  head_number:u32,
  number:u16,
}

impl Req for ReqReadWords {
  fn serialize(&self) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(30);
    //副帧头
    for &i in &REQUSET {
      out.push(i);
    }
    //目标地址
    let d = self.des.serialize();
    for &i in &d {
      out.push(i);
    }
    //请求数据长
    out.push(0x0c);
    out.push(0x0);
    //保留
    for i in 0..2 {
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
    for i in 0..3 {
      out.push(h[i]);
    }
    //软元件代码
    out.push(DeviceCode::D as u8);
    //软元件点数
    let n = self.number.to_le_bytes();
    for i in 0..2 {
      out.push(n[i]);
    }

    out
  }
}

#[test]
fn test_req_read_words_serialize() {
  let mut d: ReqReadWords = ReqReadWords {
    des: Destination::new(),
    device: DeviceWord::D,
    head_number: 1,
    number: 1,
  };

  print!("ReqReadWords:");
  for i in d.serialize() {
    print!(" {:#04X}", i);
  }
}


//批量读响应(字软元件)
struct ResReadWords {
  des: Destination,
  data: Vec<u16>,
}

impl Res for ResReadWords {
  fn deserialization(&mut self, data: &[u8]) -> std::result::Result<(), ()> {
    if data.len() < 11 {
      return Err(());
    }
    //检查副帧头
    if data[0] != RESPONSE[0] || data[1] != RESPONSE[1] {
      return Err(());
    }
    //检查地址
    let r = self.des.deserialization(&data[2..=6]);
    if r == Err(()) {
      return Err(());
    }
    //获取响应数据长
    let l: u16 = u16::from_le_bytes(data[7], data[8]);
    if data.len() != (l as usize + 9) || (l % 2 == 1) {//l需为偶数
      return Err(());
    }
    //检查结束代码
    if data[9] != 0 || data[10] != 0 {
      self.data.clear();
      return Ok(());
    }
    //拷贝数据
    for i in 0..(l / 2 - 1) {
      let v = u16::from_le_bytes(ata[11 + i * 2], data[12 + i * 2]);
      self.data.push(v);
    }
    return Ok(());
  }
}


//批量读请求(位软元件)
struct ReqReadBits{
  des:Destination,
  device:DeviceBit,
  head_number:u32,
  number:u16,
}




