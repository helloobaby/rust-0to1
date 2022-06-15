//https://crates.io/crates/windows

//rust调用原生api还可以。比C#那玩意好用
//只要熟悉windows api,也不用做太多的改动

use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() {

    println!("Hello, world!");

    unsafe{
    MessageBoxA(None, "hello", "hello", MB_OK);
    }


}
