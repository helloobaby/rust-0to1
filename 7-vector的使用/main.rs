#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {

    let mut vec:Vec<i32> = Vec::new();
    vec.push(7);
    let k = vec.pop(); // k是Option<T>,跟c++17的std::optional差不多
    println!("{}",k.unwrap()); // 类似c++的.value方法

    let primes: Vec<i32> = Vec::new();
    let mut primes = vec![2, 3, 5];
    println!("{:?}", primes);

    primes.push(7);
    println!("{:?}", primes);
    primes.remove(2);
    println!("{:?}", primes);

    let mut numbers = vec![2;10];
    println!("{:?}", numbers);

    const DEFAULT: bool = true;
    let values = vec![DEFAULT;8];
    println!("{:?}", values);

    numbers[5] = 8;
    println!("{:?}", numbers);

    let mut num2 : [i32;3] = [0,1,2];  //手动初始化整个数组
    let mut num2 : [i32;3] = [0;3];    //用0初始化整个数组
    num2[0] = 11;
    println!("{:?}",num2);

    for number in numbers.iter() {
        println!("{}", number * number);
    }
}