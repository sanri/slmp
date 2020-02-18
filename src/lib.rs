mod slmp_core;

use std::net::{TcpStream, ToSocketAddrs};
use crate::slmp_core::read_words;


#[test]
fn test() {
  println!("test begin");
  if let Ok(mut stream) = TcpStream::connect("192.168.10.250:2025") {
    println!("connect ok");

    let r = read_words(&mut stream,1,1);
    match r{
      Ok(d)=>{
        println!("读取到{}个寄存器",d.len());
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
