在crate.io发布自己的包步骤

1.先生成并登记的tokens
cargo login [private_key](https://crates.io/settings/tokens)

2.修改自己的包名称(必须不一样,先到先得)
.toml中package的name字段

3.增加license 
.toml中package的license字段