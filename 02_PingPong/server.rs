//create a simple listening UDP server on port 31337 which replies "pong" to every "ping" it receives
//
// use std::io::{Read, Write};
// use std::thread;
// use std::time::Duration;
// use std::sync::mpsc;
// use std::sync::mpsc::{Sender, Receiver};
// use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr};
use std::net::UdpSocket;

fn main() {
  //create a UDP socket on default address and port 31337
  let socket = UdpSocket::bind("localhost").unwrap();
  println!("Listening on: {}", socket.local_addr().unwrap());


  //create a buffer to hold the received data
  let mut buf = [0; 10];
  
    loop {
      //wait for a message to be received
      let (amt, src) = socket.recv_from(&mut buf).unwrap();
  
      //print the received message
      println!("Received: {:?} from: {}", &buf[..amt], src);
  
      //check if the message is "ping"
      if &buf[..amt] == b"ping" {
        //send "pong" back to the sender
        socket.send_to(b"pong", &src).unwrap();
      }
    }
}
