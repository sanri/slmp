use async_std::{io::timeout, net::TcpStream};
use std::net::{Shutdown, SocketAddr};
use std::time::{Duration, Instant};

mod slmp_core;
use crate::slmp_core::{read_bits, read_blocks, read_words, write_bits, write_blocks, write_words};
pub use crate::slmp_core::{DeviceBit, DeviceWord};

pub struct Slmp {
    stream: Vec<TcpStream>,
}

impl Slmp {
    pub fn new() -> Slmp {
        Slmp {
            stream: Vec::with_capacity(1),
        }
    }

    pub async fn connect(&mut self, addr: &SocketAddr) -> Result<(), ()> {
        if !self.stream.is_empty() {
            return Err(());
        }

        let r = timeout(Duration::from_secs(2), async {
            TcpStream::connect(addr).await
        })
        .await;
        match r {
            Ok(stream) => {
                let _ = stream.set_nodelay(true);
                self.stream.push(stream);
                Ok(())
            }
            Err(_e) => Err(()),
        }
    }

    pub fn shutdown(&mut self) -> Result<(), ()> {
        if let Some(stream) = self.stream.first_mut() {
            let _ = stream.shutdown(Shutdown::Both);
        }
        self.stream.clear();
        Ok(())
    }

    // 批量读取字软元件
    // 读取成功返回 值数组
    // 通信正常,slmp协议返回的结束代码非零时,返回 Err(end_code)
    // 其它错误都返回 Err(0)
    pub async fn read_words(
        &mut self,
        head_number: u32,
        dev: DeviceWord,
        number: u16,
    ) -> Result<Vec<u16>, u16> {
        if let Some(stream) = self.stream.first_mut() {
            return read_words(stream, dev, head_number, number).await;
        }
        Err(0)
    }

    // 批量读取位软元件
    // 读取成功返回 值数组
    // 通信正常,slmp协议返回的结束代码非零时,返回 Err(end_code)
    // 其它错误都返回 Err(0)
    pub async fn read_bits(
        &mut self,
        head_number: u32,
        dev: DeviceBit,
        number: u16,
    ) -> Result<Vec<bool>, u16> {
        if let Some(stream) = self.stream.first_mut() {
            return read_bits(stream, dev, head_number, number).await;
        }
        Err(0)
    }

    // 批量写入字软元件
    // 写入成功返回 Ok
    // 通信正常,slmp协议返回的结束代码非零时,返回 Err(end_code)
    // 其它错误都返回 Err(0)
    pub async fn write_words(
        &mut self,
        head_number: u32,
        dev: DeviceWord,
        data: &[u16],
    ) -> Result<(), u16> {
        if let Some(stream) = self.stream.first_mut() {
            return write_words(stream, dev, head_number, data).await;
        }
        Err(0)
    }

    // 批量写入位软元件
    // 写入成功返回 Ok
    // 通信正常,slmp协议返回的结束代码非零时,返回 Err(end_code)
    // 其它错误都返回 Err(0)
    pub async fn write_bits(
        &mut self,
        head_number: u32,
        dev: DeviceBit,
        data: &[bool],
    ) -> Result<(), u16> {
        if let Some(stream) = self.stream.first_mut() {
            return write_bits(stream, dev, head_number, data).await;
        }
        Err(0)
    }

    // 批量读取多个块
    // 读取成功返回 值数组
    // 通信正常,slmp协议返回的结束代码非零时,返回 Err(end_code)
    // 其它错误都返回 Err(0)
    pub async fn read_blocks(
        &mut self,
        data: &Vec<(u32, DeviceWord, u16)>,
    ) -> Result<Vec<Vec<u16>>, u16> {
        if let Some(stream) = self.stream.first_mut() {
            return read_blocks(stream, data).await;
        }
        Err(0)
    }

    // 批量写多个块 (D软元件)
    // 写入成功返回 Ok
    // 通信正常,slmp协议返回的结束代码非零时,返回 Err(end_code)
    // 其它错误都返回 Err(0)
    pub async fn write_blocks(
        &mut self,
        data: &Vec<(u32, DeviceWord, Vec<u16>)>,
    ) -> Result<(), u16> {
        if let Some(stream) = self.stream.first_mut() {
            return write_blocks(stream, data).await;
        }
        Err(0)
    }
}

async fn _test_blocks() {
    let mut slmp = Slmp::new();
    let addr = SocketAddr::from(([172, 20, 1, 33], 5432));
    match slmp.connect(&addr).await {
        Ok(_) => {
            println!("connect successful");

            let r = slmp
                .read_blocks(&vec![(1, DeviceWord::D, 10), (11, DeviceWord::D, 10)])
                .await;
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

            // async_std::task::sleep(Duration::from_secs(5)).await;

            if let Err(code) = slmp
                .write_blocks(&vec![(1, DeviceWord::D, vec![1, 1])])
                .await
            {
                println!("write blocks err code = {}", code);
            } else {
                println!("write blocks ok");
            }

            // async_std::task::sleep(Duration::from_secs(5)).await;

            let r = slmp
                .read_blocks(&vec![(1, DeviceWord::D, 10), (11, DeviceWord::D, 10)])
                .await;
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

            let _ = slmp.shutdown();
            println!("connect shutdown");
        }
        Err(..) => {
            println!("connect fault");
        }
    }
}

#[test]
fn test_blocks() {
    let now_time = Instant::now();
    async_std::task::block_on(_test_blocks());
    let time = now_time.elapsed().as_millis();
    println!("test blocks time = {}ms", time);
}

async fn _test_words() {
    let mut slmp = Slmp::new();
    let addr = SocketAddr::from(([172, 20, 1, 33], 5432));
    match slmp.connect(&addr).await {
        Ok(_) => {
            println!("connect successful");

            let r = slmp.read_words(1, DeviceWord::D, 10).await;
            match r {
                Ok(v_list) => {
                    print!("read words ok. [ ");
                    for v in v_list {
                        print!("{},", v)
                    }
                    println!("]");
                }
                Err(code) => {
                    println!("read words err code = {}", code);
                }
            }

            if let Err(code) = slmp
                .write_words(
                    1,
                    DeviceWord::D,
                    vec![1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16, 9u16, 10u16].as_slice(),
                )
                .await
            {
                println!("write words err code = {}", code);
            } else {
                println!("write words ok");
            }

            let _ = slmp.shutdown();
            println!("connect shutdown");
        }
        Err(..) => {
            println!("connect fault");
        }
    }
}

#[test]
fn test_words() {
    let now_time = Instant::now();
    async_std::task::block_on(_test_words());
    let time = now_time.elapsed().as_millis();
    println!("test words time = {}ms", time);
}

async fn _test_bits() {
    let mut slmp = Slmp::new();
    let addr = SocketAddr::from(([172, 20, 1, 33], 5432));
    match slmp.connect(&addr).await {
        Ok(_) => {
            println!("connect successful");

            let r = slmp.read_bits(3, DeviceBit::M, 9).await;
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

            if let Err(code) = slmp
                .write_bits(
                    3,
                    DeviceBit::M,
                    vec![true, false, true, true, false, true, true, true, false].as_slice(),
                )
                .await
            {
                println!("write bits err code = {}", code);
            } else {
                println!("write bits ok");
            }

            let _ = slmp.shutdown();
            println!("connect shutdown");
        }
        Err(..) => {
            println!("connect fault");
        }
    }
}

#[test]
fn test_bits() {
    let now_time = Instant::now();
    async_std::task::block_on(_test_bits());
    let time = now_time.elapsed().as_millis();
    println!("test bits time = {}ms", time);
}
