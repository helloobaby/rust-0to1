src
│  help.rs
│  install.rs
│  main.rs
│
├─install
│  │  cmd.rs
│  │  fix.rs
│  │
│  └─fix
│          util.rs
│
├─test
│      test.rs
│
└─util
	

#### 这种代码结构下,如何在src目录下的main.rs去调用fix目录下的util.rs?

如果main.rs去调用同级下面的help.rs的话,是不需要新建一个叫main的文件夹,然后下面放一个help.rs的，只需要同级目录下新建一个help.rs就可以了。(这是main.rs这个文件的特殊之处)。



如果main.rs要去调用install文件夹下的fix.rs?



后面我看了一些代码稍微多点的rust项目，我发现都是把通用功能留在最外层，比如把util搞到最外层，也就是main.rs同层，因为从里面调外面的直接use xx;就行了。



这个帖子说的好像不行

https://rustcc.cn/article?id=322cd314-ae19-4ce3-bcd2-0f8b55d05aa3

