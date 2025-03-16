# Examples

## 环境安装
- 安装trunk服务模块
```
cargo install trunk --locked
```
- 安装wasm32-unknown-unknown
```
rustup target add wasm32-unknown-unknown
```

## 运行服务
```
trunk serve --open
```

## 运行监听源码目录
```
trunk serve --watch ../core  --open
```