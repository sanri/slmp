use std::net::TcpStream;
use std::net::Shutdown::Both;

mod slmp_core;

pub struct Slmp{
  con: Option<TcpStream>,
  buffer:Vec<u8>,
}

// 建立连接


// 批量读取字软元件
pub fn read_words(){

}

// 批量写入字软元件


#[test]
fn test() {
  let mut slmp = Slmp {
    con: None,
    buffer: Vec::with_capacity(512)
  };

  if let Ok(stream) = TcpStream::connect("192.168.10.250:2025"){
    slmp.con = Some(stream);
    println!("connect ok");
  }else{
    println!("connect error");
    return;
  }

  slmp.con.unwrap().shutdown(Both);
  println!("connect shutdown");
}
