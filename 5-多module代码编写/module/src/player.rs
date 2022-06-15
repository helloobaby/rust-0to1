//
//文件默认作用域为pub player
//这里相当于又嵌套了一个pub player
//

pub mod player{
pub fn play_movie(name: &str) {
    println!("Playing movie {}", name);
}

pub fn play_audio(name: &str) {
    println!("Plaing audio {}", name);
}
}