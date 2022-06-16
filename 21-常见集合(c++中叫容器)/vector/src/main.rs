use std::vec;

fn main() {
    //两种初始化的方式

    let v1 = vec![1,2,3,4];
    let v2 : Vec<u8>= Vec::new();

    //读取vector中的元素
    //get的话可以做错误处理,[]访问错的话直接panic
    //c++的at和[]都是返回原元素的引用,rust也一样

    println!("{} {}",v1[0],v1.get(1).unwrap());

    let mut k = v1[0];
    k = k+10;
    println!("{}",k);

}
