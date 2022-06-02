use std::io;

fn main() {
    println!("guess game!\n");

    //rust 使用let分配变量,默认是const的,mut表示mutant,可变的
    //String是标准库的字符串类型,utf-8的

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to input");

    println!("you guess : {}",guess);

}
