//
//rust 的print类似于c++17的format
//不过最早的format应该是python的吧
//

fn main() {
    let a = "123";
    let b = 1;
    let c = 1.2;
    let d = 'a';
    let e = 16;
    let name = "sbb";
    let age = "21";

    const test_i32:i32 = 2; //const不能自动推断,必须显式声明

    println!("{}\n{}\n{}\n{}\n0x{:x}",a,b,c,d,e);
    println!("{:?}",[name,age]); //["sbb", "21"]

    println!("{} {}\n",b/e,b%e);//除 取模
}
