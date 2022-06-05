fn main() {
    let str = String::from("rust is good? 中文测试\n");
    let slice = &str[0..14]; //[0,14) 跟c++的迭代器概念是一样的
    let chinese = &str[14..];

    //
    //如果索引不在一个字符的边界上,会出not a char boundary异常
    //
    let chinese2 = &str[17..];

    println!("{}",slice);
    println!("{}",chinese);
    println!("{}",chinese2);
}
