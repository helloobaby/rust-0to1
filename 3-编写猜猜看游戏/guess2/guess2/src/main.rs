use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);
    println!("Please input your guess.");

    //let t;

    let mut guess = String::new(); //guess此时是一个字符串类型

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    println!("You guessed: {}", guess);
    
    //
    //将guess从字符串转化为u32类型

    //
    //expect是错误处理,似乎rust强制要求错误处理

    //这里rust有个比c++好的地方，就是能够复用变量名
    //

    //:u32 告诉编译器前面的变量是什么类型

    let guess : u32 = guess.trim().parse().expect("Please type a number!\n"); //将guess转化为u32类型
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    println!("Guess the number!");


}