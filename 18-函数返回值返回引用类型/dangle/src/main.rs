//
//在c++中这种是可以编译的，但是都知道这是错误的
//rust中会编译报错(need lifetime parameter)
//


fn test() -> &std::string::String {
    let s = std::string::String::from("s: &str");
    s
}

fn main() {
    println!("Hello, world!");
}
