use std::string::{String};


fn main() {
    //最简单的初始化String的方式
    
    let mut s = String::from("hello world");

    println!("{}",s.len());
    println!("{}",s.is_empty());
    
    println!("{}",s.find('h').unwrap());
    println!("{}",s.find("hello").unwrap());

    s.push_str("123");
    s.push('1');
    println!("{}",s);

    //将两个字符串相加
    let s1 = String::from("sbb isisisisis");
    
    //https://doc.rust-lang.org/src/alloc/string.rs.html#2203
    //上面链接是'+'operator的实现，可以看到需要一个字符串的引用
    s = s + &s1;
    println!("{}",s);
    
    //类似于C的printf  c++20的format
    s = format!("{} {} {}","hello ","123","k");
    println!("{}",s);

    /*
    rust不提供以索引形式返回字符。
    因为rust只考虑到了索引是按字节访问的，而utf-8并不是每个字符都是一个字节的
    但是其实索引可以跟字符一对一(但是这样要每次判断字符的字节数，浪费性能)
    因此为了避免字符的边界不是一个字节，杜绝了用字符索引的方式。 
    */
    //s[0];

    //这里不加mut会报错,说明下面的next会改变自己,也就说
    //它内部维护一个指针,每次next会迭代
    let mut iter = s.chars();  
    let item1 = iter.next(); //第一次调用返回第一个字符(不是指向第一个字符的迭代器)
    let item2 = iter.next(); //如果next返回None的话 
    let item3 = iter.next_back(); //返回倒数第一个字符

    //c = ((c as u8)+1)as char;
    
    println!("{}",item1.unwrap());
    println!("{}",item2.unwrap());
    println!("back {}",item3.unwrap());


    //find返回索引，而不是迭代器
    let pos = s.find('h');

    //如何修改std::string::String?
    //https://blog.csdn.net/linysuccess/article/details/123764094
    s.remove(pos.unwrap());
    s.insert(pos.unwrap(), 's');
    println!("{}",s);




















    

}
