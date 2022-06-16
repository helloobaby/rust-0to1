/*

option这东西c++17也有。
一般很多函数都会用特殊值来表示函数的失败或成功。

熟悉win内核开发的知道用0(不考虑大于0)来表示成功，其他都是错误码。
option就解决了一定需要一个特殊值作为函数返回值来代表成功的情况。
然后如果没有option，很多函数就要这样写f(&a);多用一个变量a来接收结果,因为返回值不能代表结果


*/

struct k{
    age : u32,
}

fn test(param : k) -> std::option::Option<k>{
    
    if param.age != 0 {
        println!("age : {}",param.age);
        return Some(k{age : param.age});//成功的话用Some构造一个Option
    }

    None
}


fn main() {
    let t  = k{age : 12};
    let ret = test(t);
    
    //如果option为空的话，unwrap会panic，这个和c++是一样的
    println!("{}",ret.unwrap().age);
}
