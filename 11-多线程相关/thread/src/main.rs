use std::thread;

fn test(){
    println!("thread1");
}

fn main() {
    
    //这种线程自动detach的
    std::thread::spawn(test);

    //这种的线程创建出来会是就绪状态的,需要join
    let handle  = std::thread::spawn(move||{
        println!("thread2");
    });
    handle.join();
}
