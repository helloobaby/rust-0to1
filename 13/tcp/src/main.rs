//
//@author sbb
//最简易rust 的单线程tcp server
//

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;

fn handle_client(stream :&mut TcpStream){
    let mut buffer : [u8;10] = [0;10]; 
    println!("new client ip : {} ",stream.peer_addr().unwrap());

    loop{
    let bytes_read = stream.read(&mut buffer[..]).unwrap();
    if bytes_read == 0{
        println!("client close");
        break;
    }
    else{
        println!("client send ：{:?}",buffer); //print raw hex
        buffer[0] = 'o' as u8;
        buffer[1] = 'k' as u8;
        buffer[2] = 0;

        stream.write(&buffer[..bytes_read]).unwrap();
    }

    }
}

fn main(){

    let listener = TcpListener::bind("0.0.0.0:8889").unwrap();
    
    for stream in listener.incoming() {
        handle_client(&mut stream.unwrap());
    }

}
