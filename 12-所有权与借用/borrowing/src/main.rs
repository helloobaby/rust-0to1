
fn main() {
    
    let vec = vec![1,2,3];
    println!("{:?}",vec);

    let vec2 = &vec; //这里就要加&，表示借用，而不是所有权的转让
    println!("{:?}",vec);


}
