use std::arch::asm;

fn main() {
    println!("Hello, world!");

    //rust的链接器是用的Visual Studio的(link.exe)

    unsafe{
    asm!("
    mov rax,1;
    mov rbx,1;
    call main;
    ");
}
}

