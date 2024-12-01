# 使用官方的 Rust 镜像作为基础镜像
FROM rust:latest

# 设置工作目录
WORKDIR /usr/src/openRD

# 将 Cargo.toml 和 Cargo.lock 复制到工作目录
COPY Cargo.toml Cargo.lock ./

# 构建依赖项以缓存它们
RUN cargo build --release && cargo clean

# 将项目的源代码复制到工作目录
COPY . .

# 构建项目
RUN cargo build --release

# 设置运行时的入口点
CMD ["./target/release/openRD"]