### rust的安装(ubuntu18)
##### linux

*sudo apt install rustc*

##### windows

C:\Users\asdf>rustc --version
error: no override and no default toolchain set

C:\Users\asdf>$ rustup default stable
'$' 不是内部或外部命令，也不是可运行的程序
或批处理文件。

C:\Users\asdf>rustup default stable
info: syncing channel updates for 'stable-x86_64-pc-windows-msvc'
error: could not download file from 'https://static.rust-lang.org/dist/channel-rust-stable.toml.sha256' to 'C:\Users\asdf\.rustup\tmp\0noybowte1r3xs8m_file': failed to make network request: error sending request for url (https://static.rust-lang.org/dist/channel-rust-stable.toml.sha256): operation timed out: operation timed out

**timed out要开VPN**

### 安装完成之后

rustc --version 查看版本

### 编译
rustc 1.rs