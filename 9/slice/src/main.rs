fn main() {
    let str = String::from("rust is good? 中文测试\n");
    let slice = &str[0..14]; //[0,14) 跟c++的迭代器概念是一样的
    let chinese = &str[14..];

    //
    //如果索引不在一个字节(char)的边界上,会出not a char boundary异常
    //c++的索引是按字符来的
    //
    let chinese2 = &str[17..];

    //如何获得指定字符的索引? 类似c++的std::string::find
    println!("charac index is {}",str.find('中').unwrap());

    //如果出现option为空的话
    //c++提前要用if判断的
    //rust用match匹配,match后面的case要跟这个元素的类型(Option? Result?)相匹配
    let op = str.find('嗨');
    match op{
        None => {println!("no 嗨\n");}
        Some(op) => {println!("index is {}",op);}
    }

    println!("{}",slice);
    println!("{}",chinese);
    println!("{}",chinese2);
}
