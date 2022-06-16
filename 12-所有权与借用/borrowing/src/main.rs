
fn test(param : &mut Vec<u32>){
    println!("{:?}",param);
    param[0] = param[0]+1;

}

fn main() {
    
    let mut vec = vec![1,2,3];
    println!("{:?}",vec);

    let mut vec2 = &vec; //这里就要加&，表示借用，而不是所有权的转让
    println!("{:?}",vec);

    let mut vec3 = &mut vec;

    //   rust中的引用默认是常量的，无法修改
    //vec2[0] = vec2[0]+1;   

    
    //下面这行代码是错的。因为同一时间对一个数据产生了2个可变引用
    //
    //test(&mut vec2);
    
    
    test(&mut vec3); 
}
