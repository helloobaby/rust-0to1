use std::vec::Vec;

fn test(vec : &Vec<i32>){ 
    println!("{:?}",vec);
}

fn main() {
    let mut vec : Vec<i32> = Vec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("vec : {:?}",vec);

    //vec2 borrow vec,所以在vec被销毁的时候，
    //所有权又返回给vec
    let vec2 = &vec;

    println!("{:?}",vec2);
    test(vec2);

    
    println!("{:?}",vec); 
}
