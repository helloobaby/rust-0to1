### 首先引入项目依赖

PS C:\Users\asdf\Desktop\rust(learn from0to1)\3\guess2\guess2> cargo build
    Blocking waiting for file lock on package cache

### 上面那个问题，首先找到.cargo文件夹,然后把当前目录的一个叫.package-cache删掉就行了

PS C:\Users\asdf\Desktop\rust(learn from0to1)\3\guess2\guess2> cargo build
    Updating crates.io index
  Downloaded cfg-if v1.0.0
  Downloaded rand_core v0.6.3
  Downloaded ppv-lite86 v0.2.16
  Downloaded rand_chacha v0.3.1
  Downloaded getrandom v0.2.6
  Downloaded rand v0.8.5
  Downloaded 6 crates (182.5 KB) in 2.83s
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.16
   Compiling getrandom v0.2.6
   Compiling rand_core v0.6.3
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guess2 v0.1.0 (C:\Users\asdf\Desktop\rust(learn from0to1)\3\guess2\guess2)
    Finished dev [unoptimized + debuginfo] target(s) in 57.57s