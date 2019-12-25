use std::net::{IpAddr,Ipv4Addr};

struct Request{
  network:u8,
  station:u8,
  module:u16,
  multidrop_station:u8,
}

struct Response{

}