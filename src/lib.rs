mod slmp_core;
use std::ffi::CStr;
use std::net::{TcpStream, Shutdown, ToSocketAddrs, SocketAddr};
use crate::slmp_core::{read_words, write_words, read_blocks, write_blocks};
use std::os::raw::c_char;
use std::ptr::null_mut;
use std::time::Duration;


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

  pub fn shutdown(self)->Result<(),()> {
    if let Ok(_) = self.stream.shutdown(Shutdown::Both) {
      return Ok(());
    } else {
      return Err(());
    }
  }

  // 批量读取字软元件(D软元件）
  // 读取成功返回 值数组
  // 通信正常，lsmp协议返回的结束代码非零时，返回 Err(end_code)
  // 其它错误都返回 Err(0)
  pub fn read_words(&mut self, head_number: u32, number: u16) -> Result<Vec<u16>, u16> {
    return read_words(&mut (self.stream), head_number, number)
  }

  // 批量写入字软元件 (D软元件）
  // 写入成功返回 Ok
  // 通信正常，lsmp协议返回的结束代码非零时，返回 Err(end_code)
  // 其它错误都返回 Err(0)
  pub fn write_words(&mut self, head_number: u32, data: &[u16]) -> Result<(), u16> {
    return write_words(&mut (self.stream), head_number, data);
  }

  // 批量读取多个块 (D软元件)
  // 读取成功返回 值数组
  // 通信正常，lsmp协议返回的结束代码非零时，返回 Err(end_code)
  // 其它错误都返回 Err(0)
  pub fn read_blocks(&mut self, data: &Vec<(u32, u16)>) -> Result<Vec<Vec<u16>>, u16> {
    return read_blocks(&mut (self.stream), data);
  }

  // 批量写多个块 (D软元件)
  // 写入成功返回 Ok
  // 通信正常，lsmp协议返回的结束代码非零时，返回 Err(end_code)
  // 其它错误都返回 Err(0)
  pub fn write_blocks(&mut self, data: &Vec<(u32, Vec<u16>)>) -> Result<(), u16> {
    return write_blocks(&mut(self.stream), data);
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

#[no_mangle]
extern "C" fn slmp_read_words(
  stream: &mut TcpStream, head_number:u32, number:u16, data: *mut u16) ->i32 {
  if data.is_null() {
    return -1;
  }
  let mut out: i32 = 0;
  let r = read_words(stream, head_number, number);
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

#[no_mangle]
extern "C" fn slmp_write_words(
  stream:&mut TcpStream, head_number:u32,number:u16,data:*const u16) ->i32 {
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
  let r = write_words(stream, head_number, &d);
  match r {
    Ok(_) => { out = 0 },
    Err(0) => { return -1 },
    Err(code) => { return code as i32 }
  }
  return out;
}


#[test]
fn testblocks() {
  println!("app start");
  let addr = SocketAddr::from(([192, 168, 10, 250], 2025));
  match Slmp::connect(&addr) {
    Ok(mut slmp) => {
      println!("connect successful");

      let r = slmp.read_blocks(&vec![(1, 10), (11, 10)]);
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

      if let Err(code) = slmp.write_blocks(&vec![(1, vec![1, 1])]){
        println!("write blocks err code = {}",code);
      }else {
        println!("write blocks ok");
      }

      let r = slmp.read_blocks(&vec![(1, 10), (11, 10)]);
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


