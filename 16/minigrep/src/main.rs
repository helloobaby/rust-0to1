/*
https://doc.rust-lang.org/std/env/index.html

rust程序的入口函数依然是mainCRTStartup



*/
use std::env::{args};
use std::fs::{read};

fn main() {
    let arg_list = args();
    for o in arg_list{
        println!("exe name : {}",o);
    }

    //在终端cargo run的时候,要确保这个1.bin在当前终端目录下
    //也就是ls 的时候看得见

    let file = read("./1.bin");
    let str = std::string::String::from_utf8(file.unwrap());
    println!("{:?}",str);
    


}
