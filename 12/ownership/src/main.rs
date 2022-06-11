use std::vec::Vec;

fn test(vec : &Vec<i32>){ //
    println!("{:?}",vec);
}

fn main() {
    let mut vec : Vec<i32> = Vec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("vec : {:?}",vec);

    let vec2 = vec; //rust的 = 不是c++的拷贝赋值运算符,直接就是移动语义(std::move)

    //println!("{:?}",vec);  //已经使用不了vec了,因为所有权已经转让

    println!("{:?}",vec2);
    test(&vec2);

    println!("{:?}",vec2); //Ⅱ
}
