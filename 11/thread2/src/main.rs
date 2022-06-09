use std::thread;

//我还以为rust很方便就能实现线程安全，这玩意比c++还麻烦
//rust只是能在编译的时候告诉你会出现线程安全这个问题,但是实现起来并没有优势
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

//rust的全局变量最好是全大写

//static默认是const的，和c++不一样
//static a : u64 = 1; 

//这样rust编译器会提示数据竞争引起未定义行为，也就是不是线程安全了,编译不通过
//static mut a : u64 = 1;

static a : AtomicUsize  =  AtomicUsize::new(0);

fn main() {
    let handle1 = std::thread::spawn(move||{
        for i in 0..100000{
            a.fetch_add(1,Ordering::SeqCst);
     }
    });    
    let handle2 = std::thread::spawn(move||{
        for i in 0..100000{
        a.fetch_add(1,Ordering::SeqCst);
 }
 });   
    handle1.join();
    handle2.join();

    println!("{}",a.load(Ordering::SeqCst));
}
