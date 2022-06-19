
//引入pub type Result<T> = Result<T, Error>;
use std::io::Result;
use std::string::String;
use std::fs::File;
fn main() -> Result<()>{
    //pub fn open<P: AsRef<Path>>(path: P) -> Result<File>
    let f = File::open("1.txt");

    //有两种判断返回值是否有效的方式

    //
    if f.is_err(){
        println!("invalid");
    }

    //这个api在实验性版本,稳定版本里用不了
    //let error : String= f.into_err();
    
    match f {
        Ok(File) => println!("ok"),
        Err(str) => println!("error : {}",str),
    }
    Ok(())
}
