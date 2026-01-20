use bit_vec::BitVec;
use std::collections::HashSet;
use serde_json::{Map,Value};
use sha2::{Digest, Sha256};
use std::convert::TryInto;
use std::env;
use std::fmt;
use std::str;
use std::vec;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::copy;
use std::mem::transmute;
use std::net::{SocketAddr, UdpSocket};
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::time::{Duration, SystemTime};



fn walk_object(name: &str, x: &Value, result: &mut Vec<String>) {
    let Value::Object(x) = x else { return };
    println!("name {:?}", name);
    println!("value {:?}", x);
    result.push(name.to_string());
    for (name, field) in x {
    println!("name {:?}", name);
    println!("field {:?}", field);
        walk_object(&name, field, result);
    }
}


fn main() -> Result<(), std::io::Error> {
    let mut peers = &mut HashSet::new();
    let socket = UdpSocket::bind("0.0.0.0:34254")?;
    loop {
        let mut buf = [0; 0x10000]; 
        let (_amt, src) = socket.recv_from(&mut buf).expect("socket err");
		let object: Vec<Value> = serde_json::from_slice(&buf[0.._amt]).unwrap();
    peers.insert(src);
    for message in  &object {
        println!("type {}", message);
        println!("type {}", message["message_type"]);
        match message["message_type"].as_str().unwrap() {
            "Please send peers." => send_peers(&socket,src,&peers),
            "peers:" => receive_peers(src,peers),
            _ => (),
            };
		let mut result = vec![];
		walk_object("rot", message, &mut result);
		println!("{:?}", result);
    }}
    Ok(())

}

fn send_peers(socket: &UdpSocket, src: SocketAddr , peers: &HashSet<SocketAddr>) -> (){
		println!("sending peers {:?}", peers);
      let p: Vec<SocketAddr>  = peers.into_iter().cloned().collect();

      let mut message = Map::new();
      message.insert("message_type".to_owned(),serde_json::to_value(("peers:")).unwrap());
      message.insert("peers".to_owned(),serde_json::to_value(p).unwrap());
		let message_bytes: Vec<u8> = serde_json::to_vec(&message).unwrap();
		println!("sending peers {:?}", str::from_utf8(&message_bytes));
        socket.send_to(&message_bytes,src);
    }
fn receive_peers( src: SocketAddr , peers: &HashSet<SocketAddr>) -> (){
    }
