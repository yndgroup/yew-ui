# 代码发布流程

- 去官网`https://crates.io/`注册账号
- 选择账号 > 设置 > 账号设置 > API Tokens > New API Token > 填写信息会生成一个token
```
cioS2bCA1TXhMk7Q22S2bCA1TXhMkS2bCA1TXhMk
```
- 登录
```
cargo login cioS2bCA1TXhMk7Q22S2bCA1TXhMkS2bCA1TXhMk
```
- 推送代码
```
cargo publish -p yew-ui
```