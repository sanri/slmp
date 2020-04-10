use std::net::{TcpStream};
use std::io::prelude::*;
use std::vec;

//字软元件
#[derive(Clone,Copy)]
pub(crate) enum DeviceWord {
  D = 0xA8, //数据寄存器 D
  R = 0xAF, //文件寄存器 R
}

//位软元件
#[derive(Clone,Copy)]
pub(crate) enum DeviceBit {
  X = 0x9C, //输入继电器 X
  Y = 0x9D, //输出继电器 Y
  M = 0x90, //内部继电器 M
}


//request 请求
pub(crate) trait Req{
  //序列化
  fn serialize(&self)->Vec<u8>;
}

//response 响应
pub(crate) trait Res {
  //反序列化 Deserialization
  //如果报文结构正确，但是还不完整，返回 Ok(0)
  //如果报文结构正确并完整，返回 OK(l) l：有效报文长度
  fn deserialization(&mut self, data: &[u8]) -> Result<u16, ()>;
}

const REQUSET:[u8;2]=[0x50,0x00];
const RESPONSE:[u8;2]=[0xD0,0x00];

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
struct ReqReadWords{
  des:Destination,
  device:DeviceWord,  //字元件类型
  head_number:u32,    //元件编号
  number:u16,         //元件数量
}

impl ReqReadWords {
  fn new(dev: DeviceWord) -> ReqReadWords {
    ReqReadWords {
      des: Destination::new(),
      device: dev,
      head_number: 1,
      number: 1
    }
  }
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
    out.push(self.device as u8);
    //软元件点数
    let n = self.number.to_le_bytes();
    for i in 0..2 {
      out.push(n[i]);
    }

    out
  }
}

//批量读响应(字软元件)
struct ResReadWords {
  des: Destination,
  end_code:u16,   //结束代码
  data: Vec<u16>, //数据
}

impl ResReadWords {
  fn new() -> ResReadWords {
    ResReadWords {
      des: Destination::new(),
      end_code: 0,
      data: Vec::with_capacity(128)
    }
  }
}

impl Res for ResReadWords {
  fn deserialization(&mut self, data: &[u8]) -> Result<u16, ()> {
    if data.len() < 11 {
      return Ok(0);
    }
    //检查副帧头
    if data[0] != RESPONSE[0] || data[1] != RESPONSE[1] {
//      println!("副帧头 错误");
      return Err(());
    }
    //检查地址
    let r = self.des.deserialization(&data[2..=6]);
    if r == Err(()) {
      return Err(());
    }
    //获取响应数据长
    let l: u16 = u16::from_le_bytes([data[7], data[8]]);
    //报文长度
    let len: u16 = l + 9;
    if data.len() < (len as usize) {
      return Ok(0);
    }

    //检查结束代码
    self.end_code = u16::from_le_bytes([data[9], data[10]]);
    if self.end_code != 0 {
      self.data.clear();
      return Ok(len);
    }
    //拷贝数据
    for i in 0..(l / 2 - 1) {
      let p1 = 11 + i * 2;
      let p2 = p1 + 1;
      let ul: [u8; 2] = [data[p1 as usize], data[p2 as usize]];
      let v = u16::from_le_bytes(ul);
      self.data.push(v);
    }
    return Ok(len);
  }
}

//批量写请求(字软元件)
struct ReqWriteWords{
  des:Destination,
  device:DeviceWord,
  head_number:u32,
  data: Vec<u16>,
}

impl ReqWriteWords {
  fn new(dev: DeviceWord) -> ReqWriteWords {
    ReqWriteWords {
      des: Destination::new(),
      device: dev,
      head_number: 1,
      data: vec![]
    }
  }
}

impl Req for ReqWriteWords {
  fn serialize(&self) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(32);
    //副帧头
    for &i in &REQUSET {
      out.push(i);
    }
    //目标地址
    let des = self.des.serialize();
    for i in &des {
      out.push(i.clone());
    }
    //请求数据长,先占位
    out.push(0x0);
    out.push(0x0);
    //保留
    for i in 0..2 {
      out.push(0);
    }
    //指令
    out.push(0x01);
    out.push(0x14);
    //子指令
    out.push(0x00);
    out.push(0x00);
    //起始软元件编号
    let h = self.head_number.to_le_bytes();
    for i in 0..3 {
      out.push(h[i]);
    }
    //软元件代码
    out.push(self.device as u8);
    //软元件点数
    let n :[u8;2]= (self.data.len() as u16).to_le_bytes();
    for i in 0..2 {
      out.push(n[i]);
    }
    //数据
    for v in &self.data {
      let l = v.to_le_bytes();
      for i in 0..2 {
        out.push(l[i]);
      }
    }
    //修改数据长
    let l = (out.len() - 9) as u16;
    let lv = l.to_le_bytes();
    out[7] = lv[0];
    out[8] = lv[1];
    out
  }
}

//批量写响应(字软元件)
struct ResWriteWords {
  des: Destination,
  end_code:u16,    //结束代码
}

impl ResWriteWords{
  fn new()->ResWriteWords{
    ResWriteWords{
      des: Destination::new(),
      end_code: 0
    }
  }
}

impl Res for ResWriteWords {
  fn deserialization(&mut self, data: &[u8]) -> std::result::Result<u16, ()> {
    if data.len() < 11 {
      return Ok(0);
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
    let l: u16 = u16::from_le_bytes([data[7], data[8]]);

    //报文长度
    let len: u16 = l + 9;
    if data.len() < (len as usize) {
      return Ok(0);
    }

    //检查结束代码
    self.end_code = u16::from_le_bytes([data[9], data[10]]);

    return Ok(len);
  }
}

//批量读多个块请求(字软元件)
//字软元件块数 <= 120 块
//总字软元件点数 <= 960 点
struct ReqReadBlockWord{
  des: Destination,
  data:Vec<(u32,DeviceWord,u16)>,//字软元件编号，软元件代码，软元件点数
  //没有实现位软元件
}

impl ReqReadBlockWord {
  fn new() -> ReqReadBlockWord {
    ReqReadBlockWord {
      des: Destination::new(),
      data: vec![]
    }
  }
}

impl Req for ReqReadBlockWord {
  fn serialize(&self) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(128);
    //副帧头
    for &i in &REQUSET {
      out.push(i);
    }
    //目标地址
    let d = self.des.serialize();
    for &i in &d {
      out.push(i);
    }
    //请求数据长,占位
    out.push(0x0);
    out.push(0x0);
    //保留
    for i in 0..2 {
      out.push(0);
    }
    //指令
    out.push(0x06);
    out.push(0x04);
    //子指令
    out.push(0x00);
    out.push(0x00);
    //字软元件块数
    out.push(self.data.len() as u8);
    //位软元件块数
    out.push(0x00);
    //字软元件
    for (head_number, device, number) in &self.data {
      //起始软元件编号
      let h = head_number.to_le_bytes();
      for i in 0..3 {
        out.push(h[i]);
      }
      //软元件代码
      out.push(device.clone() as u8);
      //软元件点数
      let n = number.to_le_bytes();
      for i in 0..2 {
        out.push(n[i]);
      }
    }
    //不实现位软元件

    //修改数据长
    let l = (out.len() - 9) as u16;
    let lv = l.to_le_bytes();
    out[7] = lv[0];
    out[8] = lv[1];

    return out;
  }
}

//批量读多个块响应(字软元件)
struct ResReadBlockWord{
  des:Destination,
  req_data:Vec<(DeviceWord,u16)>,  //请求数据
  end_code:u16,           //结束代码
  data:Vec<Vec<u16>>,     //只实现了字软元件
}

impl ResReadBlockWord {
  fn new() -> ResReadBlockWord {
    ResReadBlockWord {
      des: Destination::new(),
      req_data: vec![],
      end_code: 0,
      data: vec![],
    }
  }
}

impl Res for ResReadBlockWord {
  fn deserialization(&mut self, data: &[u8]) -> Result<u16, ()> {
    if data.len() < 11 {
      return Ok(0);
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
    let l: u16 = u16::from_le_bytes([data[7], data[8]]);
    //报文长度
    let len: u16 = l + 9;
    if data.len() < (len as usize) {
      return Ok(0);
    }
    //检查结束代码
    self.end_code = u16::from_le_bytes([data[9], data[10]]);
    if self.end_code != 0 {
      self.data.clear();
      return Ok(len);
    }
    //拷贝数据
    let mut i = 0;
    for (device, number) in &self.req_data {
      let mut block: Vec<u16> = Vec::new();
      for j in 0..*number {
        let p1 = 11 + i * 2;
        let p2 = p1 + 1;
        i += 1;
        if p2 >= len {
          return Err(());
        }
        let ul: [u8; 2] = [data[p1 as usize], data[p2 as usize]];
        let v = u16::from_le_bytes(ul);
        block.push(v);
      }
      self.data.push(block);
    }
    return Ok(len);
  }
}

//批量写多个块(字软元件)
struct ReqWriteBlockWord{
  des:Destination,
  data:Vec<(u32,DeviceWord,Vec<u16>)>,//字软元件编号,软元件代码,块数据
  //位软元件不实现
}

impl ReqWriteBlockWord {
  fn new() -> ReqWriteBlockWord {
    ReqWriteBlockWord {
      des: Destination::new(),
      data: vec![]
    }
  }
}

impl Req for ReqWriteBlockWord {
  fn serialize(&self) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(128);
    //副帧头
    for &i in &REQUSET {
      out.push(i);
    }
    //目标地址
    let des = self.des.serialize();
    for i in &des {
      out.push(i.clone());
    }
    //请求数据长,先占位
    out.push(0x0);
    out.push(0x0);
    //保留
    for i in 0..2 {
      out.push(0);
    }
    //指令
    out.push(0x06);
    out.push(0x14);
    //子指令
    out.push(0x00);
    out.push(0x00);
    //字软元件块数
    out.push(self.data.len() as u8);
    //位软元件块数
    out.push(0x00);

    for (head_number, device, d) in &self.data {
      //起始软元件编号
      let h = head_number.to_le_bytes();
      for i in 0..3 {
        out.push(h[i]);
      }
      //软元件代码
      out.push(*device as u8);
      //软元件点数
      let n: [u8; 2] = (d.len() as u16).to_le_bytes();
      for i in 0..2 {
        out.push(n[i]);
      }
      //数据
      for v in d {
        let l = v.to_le_bytes();
        for i in 0..2 {
          out.push(l[i]);
        }
      }
    }

    //修改数据长
    let l = (out.len() - 9) as u16;
    let lv = l.to_le_bytes();
    out[7] = lv[0];
    out[8] = lv[1];

    return out;
  }
}

//批量写多个块响应(字软元件)
struct ResWriteBlockWord{
  des:Destination,
  end_code:u16,
}

impl ResWriteBlockWord {
  fn new() -> ResWriteBlockWord {
    ResWriteBlockWord {
      des: Destination::new(),
      end_code: 0,
    }
  }
}

impl Res for ResWriteBlockWord {
  fn deserialization(&mut self, data: &[u8]) -> Result<u16, ()> {
    if data.len() < 11 {
      return Ok(0);
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
    let l: u16 = u16::from_le_bytes([data[7], data[8]]);

    //报文长度
    let len: u16 = l + 9;
    if data.len() < (len as usize) {
      return Ok(0);
    }

    //检查结束代码
    self.end_code = u16::from_le_bytes([data[9], data[10]]);

    return Ok(len);
  }
}

// 批量读取字软元件
// 读取成功返回 值数组
// 通信正常，lsmp协议返回的结束代码非零时，返回 Err(end_code)
// 其它错误都返回 Err(0)
pub fn read_words(stream: &mut TcpStream,dev:DeviceWord, head_number:u32, number:u16) ->Result<Vec<u16>,u16> {
  let mut out = Result::Err(0);
  let mut req = ReqReadWords::new(dev);
  let mut res = ResReadWords::new();
  req.head_number = head_number;
  req.number = number;
  let msg: Vec<u8> = req.serialize();
  if let Err(_) = stream.write_all(&msg) {
    return out
  }
  let mut buffer: Vec<u8> = Vec::with_capacity(128);

  'a: loop {
    let mut b = [0u8; 128];
    if let Ok(n) = stream.read(&mut b) {
      for i in 0..n {
        buffer.push(b[i]);
      }
      match res.deserialization(&buffer) {
        Ok(0) => { //报文不完整
          continue;
        },
        Ok(n) => { //已解析出完整报文
          if res.end_code != 0 {
            out = Result::Err(res.end_code);
            break 'a;
          }
          out = Result::Ok(res.data.clone());
          break 'a;
        },
        Err(_) => { //报文结构不正确
          return out;
        }
      }
    } else {
      return out
    }
  }
  return out;
}

// 批量写入字软元件 (D软元件）
// 写入成功返回 Ok
// 通信正常，lsmp协议返回的结束代码非零时，返回 Err(end_code)
// 其它错误都返回 Err(0)
pub fn write_words(stream:&mut TcpStream,head_number:u32,data:&[u16])->Result<(),u16> {
  let mut out = Result::Err(0);
  let mut req = ReqWriteWords::new();
  let mut res = ResWriteWords::new();
  req.head_number = head_number;
  req.data = Vec::from(data);
  let msg: Vec<u8> = req.serialize();
  if let Err(_) = stream.write_all(&msg) {
    return out
  }
  let mut buffer: Vec<u8> = Vec::with_capacity(128);

  'a: loop {
    let mut b = [0u8; 128];
    if let Ok(n) = stream.read(&mut b) {
      for i in 0..n {
        buffer.push(b[i]);
      }
      match res.deserialization(&buffer) {
        Ok(0) => { //报文不完整
          continue;
        },
        Ok(n) => { //已解析出完整报文
          if res.end_code != 0 {
            out = Result::Err(res.end_code);
            break 'a;
          }
          out = Result::Ok(());
          break 'a;
        },
        Err(_) => { //报文结构不正确
          return out;
        }
      }
    } else {
      return out
    }
  }
  return out;
}

// 批量读取多个块 (D软元件)
// 读取成功返回 值数组
// 通信正常，lsmp协议返回的结束代码非零时，返回 Err(end_code)
// 其它错误都返回 Err(0)
pub fn read_blocks(stream:&mut TcpStream,data:&Vec<(u32,u16)>)->Result<Vec<Vec<u16>>,u16> {
  let mut out = Result::Err(0);
  let mut req = ReqReadBlockWord::new();
  let mut res = ResReadBlockWord::new();
  for (head_number, number) in data {
    req.data.push((*head_number, DeviceWord::D, *number));
    res.req_data.push((DeviceWord::D, *number));
  }

  let msg: Vec<u8> = req.serialize();
  if let Err(_) = stream.write_all(&msg) {
    return out
  }
  let mut buffer: Vec<u8> = Vec::with_capacity(128);

  'a: loop {
    let mut b = [0u8; 256];
    if let Ok(n) = stream.read(&mut b) {
      for i in 0..n {
        buffer.push(b[i]);
      }
      match res.deserialization(&buffer) {
        Ok(0) => { //报文不完整
          continue;
        },
        Ok(n) => { //已解析出完整报文
          if res.end_code != 0 {
            out = Result::Err(res.end_code);
            break 'a;
          }
          out = Result::Ok(res.data.clone());
          break 'a;
        },
        Err(_) => { //报文结构不正确
          return out;
        }
      }
    } else {
      return out
    }
  }
  return out;
}

// 批量写多个块 (D软元件)
// 写入成功返回 Ok
// 通信正常，lsmp协议返回的结束代码非零时，返回 Err(end_code)
// 其它错误都返回 Err(0)
pub fn write_blocks(stream:&mut TcpStream,data:&Vec<(u32,Vec<u16>)>)->Result<(),u16> {
  let mut out = Result::Err(0);
  let mut req = ReqWriteBlockWord::new();
  let mut res = ResWriteBlockWord::new();
  for (head_number, d) in data {
    req.data.push((*head_number, DeviceWord::D, d.clone()));
  }
  let msg: Vec<u8> = req.serialize();
  if let Err(_) = stream.write_all(&msg) {
    return out
  }
  let mut buffer: Vec<u8> = Vec::with_capacity(128);

  'a: loop {
    let mut b = [0u8; 256];
    if let Ok(n) = stream.read(&mut b) {
      for i in 0..n {
        buffer.push(b[i]);
      }
      match res.deserialization(&buffer) {
        Ok(0) => { //报文不完整
          continue;
        },
        Ok(n) => { //已解析出完整报文
          if res.end_code != 0 {
            out = Result::Err(res.end_code);
            break 'a;
          }
          out = Result::Ok(());
          break 'a;
        },
        Err(_) => { //报文结构不正确
          return out;
        }
      }
    } else {
      return out
    }
  }
  return out;
}



