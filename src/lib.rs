mod slmp_core;

use std::net::{TcpStream, ToSocketAddrs};
use crate::slmp_core::{read_words, write_words};


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
