# Rust编译命令
```
cargo build --release --lib
```

# C/C++编译命令

```
cargo build --features cpp --no-default-features --lib
```

头文件：[`deviceid.h`](deviceid.h)

测试脚本编译命令：
```
g++ test.cpp -o test -L./target/release -ldeviceid
```

# Python编译命令

```
cd python-bindings
maturin build --release
```

wheel在`target/wheels`下

# WASM编译命令

1. bundler模式

```
wasm-pack build --target bundler --release
```

2. web模式

```
wasm-pack build --target web --release
```

3. node模式

```
wasm-pack build --target nodejs --release
```
