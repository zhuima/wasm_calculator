## 环境准备

```rust
// 安装rust
rustc --version


// 安装wasm工具
rustup target add wasm32-unknown-unknown

// 安装编译工具

cargo install wasm-pack


// http服务器

npm install -g http-server


```

## 构建

```rust
wasm-pack build --target web
```


## 访问

```rust
http-server
```
