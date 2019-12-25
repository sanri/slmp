use std::net::{IpAddr,Ipv4Addr};

struct Destination{
  network:u8,           //网络编号
  station:u8,           //站号
  module:u16,           //模块IO编号
  multidrop_station:u8, //多点站号
}

struct Request{
  des:Destination,
}

struct Response{
  des:Destination,
}




