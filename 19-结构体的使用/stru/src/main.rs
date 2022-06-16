use std::string;

#[derive(Debug)]

struct People{
    age : u32,//c++结构体每个成员都是分号结尾，rust是逗号
    name : String, 
} //rust结构体最后也不需要';'

fn main() {

    //rust结构体初始化用'成员 : 值'来初始化
    //不同于c++ 直接花括号里直接给值然后按顺序初始化
    //不过c++也可以用'.'来指定成员进行初始化，这个和rust差不多

    let t1 = People{age : 1,name : String::from("123")};

    println!("{:?}",t1);



}
