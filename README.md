# OpenRD 国内罕见病开源社区

OpenRD 是一个专注于国内罕见病领域的开源社区项目，旨在通过开源软件和社区协作，推动罕见病研究、数据管理和患者支持的发展。

## 目录
- [项目结构](#项目结构)
- [如何启动项目](#如何启动项目)
  - [环境准备](#环境准备)
  - [构建与运行](#构建与运行)
    - [构建项目](#构建项目)
    - [运行数据库容器](#运行数据库容器)
    - [应用数据库迁移](#应用数据库迁移)
    - [启动应用](#启动应用)
  - [如何判断启动成功](#如何判断启动成功)
    - [日志输出](#日志输出)
    - [访问 Hello World 端点](#访问-hello-world-端点)
- [SQL 变更管理](#sql-变更管理)
- [自动删除未使用的导入](#自动删除未使用的导入)
- [贡献指南](#贡献指南)
- [联系我们](#联系我们)

## 项目结构

```
src
 handlers # 处理器 
 models # 结构体 
 routes # 路由 
 services # 主要业务处理 
 utils # 工具类 
 main.rs # 启动类
```

## 如何启动项目

### 环境准备

1. **安装 Rust 工具链**

  请参考 [Rust 官方安装指南](https://www.rust-lang.org/tools/install) 进行安装。

2. **安装 Docker**

  请参考 [Docker 官方安装指南](https://docs.docker.com/get-docker/) 进行安装。

3. **配置环境变量**

  创建一个 `.env` 文件在项目根目录，添加必要的环境变量，例如：

  ```env
  DATABASE_URL=mysql://user:password@localhost:3306/opendr_db
  JWT_SECRET=your_jwt_secret
  ```

### 构建与运行

#### 1. 构建项目

在项目根目录运行以下命令以构建项目：
```
cargo build --release
```
#### 2.运行数据库容器

使用 Docker 启动数据库（以 MySQL 为例）：

```
docker run --name opendr_db -e MYSQL_ROOT_PASSWORD=password -e MYSQL_DATABASE=opendr_db -e MYSQL_USER=user -e MYSQL_PASSWORD=password -p 3306:3306 -d mysql:latest
```

#### 3.应用数据库迁移

使用 Diesel 进行数据库迁移：

```
diesel setup
diesel migration run
```

#### 4. 启动应用

运行以下命令启动应用：

```
cargo run --release
```
或者使用 Docker Compose（如果有配置）：

```
docker-compose up --build
```
### 如何判断启动成功
启动成功后，您可以通过以下方式确认：

#### 日志输出

观察控制台日志，应该看到类似以下的信息：

```
Starting HTTP server at http://127.0.0.1:8080
```

#### 访问 Hello World 端点

访问 `Hello World `端点以确认路由配置正确：
```
curl http://localhost:8080/hello
```
应返回：
```
Hello, World!
```
## 贡献指南

欢迎贡献者参与开发！请遵循以下步骤：

1. Fork 本仓库
2. 创建一个新的分支 (`git checkout -b feature/你的功能`)
3. 提交你的更改 (`git commit -m '添加新功能'`)
4. 推送到分支 (`git push origin feature/你的功能`)
5. 创建一个新的 Pull Request

## 联系我们

如果您有任何问题或建议，请通过以下方式联系我们：

- 邮箱：support@openrd.org
- [GitHub Issues](https://github.com/OpenRD-web-server/issues)
