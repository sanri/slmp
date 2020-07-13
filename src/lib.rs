use std::ffi::CStr;
use std::net::{TcpStream, Shutdown, ToSocketAddrs, SocketAddr};
use std::os::raw::{c_char, c_int, c_uint, c_short, c_ushort, c_uchar};
use std::ptr::null_mut;
use std::time::Duration;

mod slmp_core;
use crate::slmp_core::{read_words, write_words, read_blocks, write_blocks,read_bits,write_bits};
pub use crate::slmp_core::{DeviceBit,DeviceWord};


pub struct Slmp{
  stream:TcpStream,
}

impl Slmp {
  pub fn connect(addr: &SocketAddr) -> Result<Slmp, ()> {
    if let Ok(stream) = TcpStream::connect_timeout(addr, Duration::new(2, 0)) {
      stream.set_read_timeout(Some(Duration::new(1, 0)));
      stream.set_write_timeout(Some(Duration::new(1, 0)));
      Ok(Slmp { stream: stream })
    } else {
      Err(())
    }
  }
  
  pub fn shutdown(self) -> Result<(), ()> {
    if let Ok(_) = self.stream.shutdown(Shutdown::Both) {
      return Ok(());
    } else {
      return Err(());
    }
  }
  
  // 批量读取字软元件
  // 读取成功返回 值数组
  // 通信正常,slmp协议返回的结束代码非零时,返回 Err(end_code)
  // 其它错误都返回 Err(0)
  pub fn read_words(&mut self, head_number: u32, dev: DeviceWord, number: u16) -> Result<Vec<u16>, u16> {
    return read_words(&mut self.stream, dev, head_number, number);
  }
  
  // 批量读取位软元件
  // 读取成功返回 值数组
  // 通信正常,slmp协议返回的结束代码非零时,返回 Err(end_code)
  // 其它错误都返回 Err(0)
  pub fn read_bits(&mut self, head_number: u32, dev: DeviceBit, number: u16) -> Result<Vec<bool>, u16> {
    return read_bits(&mut self.stream, dev, head_number, number);
  }
  
  // 批量写入字软元件
  // 写入成功返回 Ok
  // 通信正常,slmp协议返回的结束代码非零时,返回 Err(end_code)
  // 其它错误都返回 Err(0)
  pub fn write_words(&mut self, head_number: u32, dev: DeviceWord, data: &[u16]) -> Result<(), u16> {
    return write_words(&mut self.stream, dev, head_number, data);
  }
  
  // 批量写入位软元件
  // 写入成功返回 Ok
  // 通信正常,slmp协议返回的结束代码非零时,返回 Err(end_code)
  // 其它错误都返回 Err(0)
  pub fn write_bits(&mut self, head_number: u32, dev: DeviceBit, data: &[bool]) -> Result<(), u16> {
    return write_bits(&mut self.stream, dev, head_number, data);
  }
  
  // 批量读取多个块
  // 读取成功返回 值数组
  // 通信正常,slmp协议返回的结束代码非零时,返回 Err(end_code)
  // 其它错误都返回 Err(0)
  pub fn read_blocks(&mut self, data: &Vec<(u32, DeviceWord, u16)>) -> Result<Vec<Vec<u16>>, u16> {
    return read_blocks(&mut self.stream, data);
  }
  
  // 批量写多个块 (D软元件)
  // 写入成功返回 Ok
  // 通信正常,slmp协议返回的结束代码非零时,返回 Err(end_code)
  // 其它错误都返回 Err(0)
  pub fn write_blocks(&mut self, data: &Vec<(u32, DeviceWord, Vec<u16>)>) -> Result<(), u16> {
    return write_blocks(&mut self.stream, data);
  }
}

#[no_mangle]
extern "C" fn slmp_connect(ip:*const c_char,port:u16) -> *mut TcpStream {
  let ip_c_str = unsafe {
    assert!(!ip.is_null());
    CStr::from_ptr(ip)
  };
  let ip_r_str = ip_c_str.to_str().unwrap();
  if let Ok(mut stream) = TcpStream::connect((ip_r_str, port)) {
    stream.set_read_timeout(Some(Duration::new(1, 0)));
    stream.set_write_timeout(Some(Duration::new(1, 0)));
    return Box::into_raw(Box::new(stream));
  } else {
    return null_mut();
  };
}

#[no_mangle]
extern "C" fn slmp_shutdown(s:*mut TcpStream) {
  if s.is_null() {
    return;
  }
  let mut stream = unsafe { *Box::from_raw(s) };
  stream.shutdown(Shutdown::Both);
}

//dev
// 1 保持寄存器 D
// 2 文件寄存器 R
#[no_mangle]
extern "C" fn slmp_read_words(
  stream: &mut TcpStream, head_number: c_uint, dev: c_ushort, number: c_ushort, data: *mut c_ushort
) ->i32 {
  if data.is_null() {
    return -1;
  }
  let mut out: i32 = 0;
  let device = match dev {
    1 => DeviceWord::D,
    2 => DeviceWord::R,
    _ => DeviceWord::D,
  };
  let r = read_words(stream, device, head_number, number);
  match r {
    Ok(r) => {
      for i in 0..number {
        unsafe {
          let p: *mut u16 = data.offset(i as isize);
          *p = r[i as usize];
        }
      }
      out = 0;
    },
    Err(0) => { out = -1; },
    Err(code) => { out = code as i32; }
  };
  return out;
}

//dev
// 1 内部继电器 M
// 2 输入继电器 X
// 3 输出继电器 Y
#[no_mangle]
extern "C" fn slmp_read_bits(
  stream: &mut TcpStream, head_number: c_uint, dev: c_ushort, number: c_ushort, data: *mut c_uchar
) ->i32 {
  if data.is_null() {
    return -1;
  }
  let mut out: i32 = 0;
  let device = match dev {
    1 => DeviceBit::M,
    2 => DeviceBit::X,
    3 => DeviceBit::Y,
    _ => DeviceBit::M,
  };
  let r = read_bits(stream, device, head_number, number);
  match r {
    Ok(r) => {
      for i in 0..number {
        unsafe {
          let p: *mut u8 = data.offset(i as isize);
          *p = if r[i as usize] { 1 } else { 0 };
        }
      }
      out = 0;
    },
    Err(0) => { out = -1; },
    Err(code) => { out = code as i32; }
  };
  return out;
}

//dev
// 1 保持寄存器 D
// 2 文件寄存器 R
#[no_mangle]
extern "C" fn slmp_write_words(
  stream:&mut TcpStream, head_number:u32,dev:u16,number:u16,data:*const u16
) ->i32 {
  if data.is_null() {
    return -1;
  }
  let mut out: i32 = 0;
  let mut d: Vec<u16> = Vec::with_capacity(number as usize);
  for i in 0..number {
    unsafe {
      let p: *const u16 = data.offset(i as isize);
      d.push(*p);
    }
  }
  let device = match dev {
    1 => DeviceWord::D,
    2 => DeviceWord::R,
    _ => DeviceWord::D,
  };
  let r = write_words(stream, device, head_number, &d);
  match r {
    Ok(_) => { out = 0 },
    Err(0) => { return -1 },
    Err(code) => { return code as i32 }
  }
  return out;
}

//dev
// 1 内部继电器 M
// 2 输入继电器 X
// 3 输出继电器 Y
#[no_mangle]
extern "C" fn slmp_write_bits(
  stream:&mut TcpStream, head_number: c_uint, dev: c_ushort, number: c_ushort, data: *const c_uchar
) ->i32 {
  if data.is_null() {
    return -1;
  }
  let mut out: i32 = 0;
  let mut d: Vec<bool> = Vec::with_capacity(number as usize);
  for i in 0..number {
    unsafe {
      let p: *const u8 = data.offset(i as isize);
      let b = if *p == 0 { false } else { true };
      d.push(b);
    }
  }
  let device = match dev {
    1 => DeviceBit::M,
    2 => DeviceBit::X,
    2 => DeviceBit::Y,
    _ => DeviceBit::M,
  };
  let r = write_bits(stream, device, head_number, &d);
  match r {
    Ok(_) => { out = 0 },
    Err(0) => { return -1 },
    Err(code) => { return code as i32 }
  }
  return out;
}

#[test]
fn testblocks() {
  let addr = SocketAddr::from(([192, 168, 0, 10], 5000));
  match Slmp::connect(&addr) {
    Ok(mut slmp) => {
      println!("connect successful");
      
      let r = slmp.read_blocks(&vec![(1, DeviceWord::D, 10), (11, DeviceWord::D, 10)]);
      match r {
        Ok(vlist) => {
          print!("read blocks ok. [ ");
          for vup in vlist {
            print!("[ ");
            for v in vup {
              print!("{}, ", v);
            }
            print!("], ");
          }
          println!("]");
        }
        Err(code) => {
          println!("read blocks err code = {}", code);
        }
      }
      
      if let Err(code) = slmp.write_blocks(&vec![(1, DeviceWord::D, vec![1, 1])]) {
        println!("write blocks err code = {}", code);
      } else {
        println!("write blocks ok");
      }
      
      let r = slmp.read_blocks(&vec![(1, DeviceWord::D, 10), (11, DeviceWord::D, 10)]);
      match r {
        Ok(vlist) => {
          print!("read blocks ok. [ ");
          for vup in vlist {
            print!("[ ");
            for v in vup {
              print!("{}, ", v);
            }
            print!("], ");
          }
          println!("]");
        }
        Err(code) => {
          println!("read blocks err code = {}", code);
        }
      }
      
      
      slmp.shutdown();
      println!("connect shutdown");
    }
    Err(..) => {
      println!("connect fault");
    }
  }
}


#[test]
fn testwords() {
  let addr = SocketAddr::from(([192, 168, 0, 10], 5000));
  match Slmp::connect(&addr) {
    Ok(mut slmp) => {
      println!("connect successful");
      
      let r = slmp.read_words(1, DeviceWord::D, 10);
      match r {
        Ok(vlist) => {
          print!("read words ok. [ ");
          for v in vlist {
            print!("{},", v)
          }
          println!("]");
        }
        Err(code) => {
          println!("read words err code = {}", code);
        }
      }
      
      if let Err(code) = slmp.write_words(1, DeviceWord::D, vec![1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16, 9u16, 10u16].as_slice()) {
        println!("write words err code = {}", code);
      } else {
        println!("write words ok");
      }
      
      slmp.shutdown();
      println!("connect shutdown");
    }
    Err(..) => {
      println!("connect fault");
    }
  }
}


#[test]
fn testbits() {
  let addr = SocketAddr::from(([192, 168, 10, 61], 5000));
  match Slmp::connect(&addr) {
    Ok(mut slmp) => {
      println!("connect successful");
      
      let r = slmp.read_bits(3, DeviceBit::M, 9);
      match r {
        Ok(vlist) => {
          print!("read bits ok. [ ");
          for v in vlist {
            print!("{},", v)
          }
          println!("]");
        }
        Err(code) => {
          println!("read bits err code = {}", code);
        }
      }
      
      if let Err(code) = slmp.write_bits(3, DeviceBit::M, vec![true, false, true, true, false, true, true, true, false].as_slice()) {
        println!("write bits err code = {}", code);
      } else {
        println!("write bits ok");
      }
      
      slmp.shutdown();
      println!("connect shutdown");
    }
    Err(..) => {
      println!("connect fault");
    }
  }
}
