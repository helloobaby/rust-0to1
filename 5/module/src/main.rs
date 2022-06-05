//
//mod 类似c++的namespace 也支持嵌套
//

mod player; //引入另一个.rs文件,标准点的说法应该是引入其他crate(module)
mod hello; 

fn main(){
    player::player::play_movie("snatch.mp4");
    player::player::play_audio("rhcp.mp3");

    hello::p();

    clean::perform_clean();
    clean::files::clean_files();
}

mod clean {
    pub fn perform_clean() {
        println!("Cleaning hdd");
    }

    pub mod files {
        pub fn clean_files() {
            println!("Removing unused files");
        }
    }
}