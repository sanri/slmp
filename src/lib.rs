mod slmp_core;
use std::ffi::CStr;
use std::net::{TcpStream,Shutdown};
use crate::slmp_core::{read_words, write_words};
use std::os::raw::c_char;
use std::ptr::null_mut;

#[no_mangle]
extern "C" fn slmp_connect(ip:*const c_char,port:u16) -> *mut TcpStream {
  let ip_c_str = unsafe {
    assert!(!ip.is_null());
    CStr::from_ptr(ip)
  };
  let ip_r_str = ip_c_str.to_str().unwrap();
  if let Ok(mut stream) = TcpStream::connect((ip_r_str, port)) {
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
fn test() {
  println!("test begin");
  if let Ok(mut stream) = TcpStream::connect("192.168.10.250:2025") {
    println!("connect ok");

    print!("read 1: ");
    let r = read_words(&mut stream,1,2);
    match r{
      Ok(d)=>{
        for v in d{
          print!("{}, ",v);
        }
        println!(" ");
      },
      Err(v)=>{
        println!("通信错误，错误码 = {}",v);
      }
    }

    print!("write 1: ");
    let r = write_words(&mut stream,1,&[1,2]);
    match r{
      Ok(_)=>{
        println!("写入成功");
      },
      Err(v)=>{
        println!("通信错误，错误码 = {}",v);
      }
    }

    print!("read 2: ");
    let r = read_words(&mut stream,1,2);
    match r{
      Ok(d)=>{
        for v in d{
          print!("{}, ",v);
        }
        println!(" ");
      },
      Err(v)=>{
        println!("通信错误，错误码 = {}",v);
      }
    }

    print!("write 2: ");
    let r = write_words(&mut stream,1,&[100,9]);
    match r{
      Ok(_)=>{
        println!("写入成功");
      },
      Err(v)=>{
        println!("通信错误，错误码 = {}",v);
      }
    }

    print!("read 3: ");
    let r = read_words(&mut stream,1,2);
    match r{
      Ok(d)=>{
        for v in d{
          print!("{}, ",v);
        }
        println!(" ");
      },
      Err(v)=>{
        println!("通信错误，错误码 = {}",v);
      }
    }

    stream.shutdown(std::net::Shutdown::Both);
    println!("connect shutdown");
  } else {
    println!("connect error");
    return;
  }

}
