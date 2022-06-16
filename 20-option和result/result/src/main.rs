/*

result::Result实际是一个enum

*/

struct k{
    age : u32,
}

fn test(param : k) -> std::result::Result<u32,std::string::String>
{
    if param.age != 0{
        return Ok(param.age);
    }

    Err(std::string::String::from("result error"))



    


}

fn main() {
    let a = k{age:3};
    let ret = test(a);

    //rust可以用match代替if
    if(ret.is_ok())
    {
        println!("{}",ret.unwrap());
    }
    else
    {
        println!("123");
    }
}