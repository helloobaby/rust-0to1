在crate.io发布自己的包步骤

1.先生成并登记的tokens
cargo login [private_key](https://crates.io/settings/tokens)

2.修改自己的包名称(必须不一样,先到先得)
.toml中package的name字段

3.增加license 
.toml中package的license字段

弄完应该还会报错,rust的报错很人性化的，还有提示，照着做就行了

crate有问题的话，每次修改完代码还要修改一下.toml里的version才能提交,
因为crates.io的原则是代码不可以删除，只能增加
