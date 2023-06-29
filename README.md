# News Recommend System - Server

## Feature

- 使用 docker 分模块部署解耦
- 后端使用 Rust 高性能实现
- 模块间调用使用 RPC 方案
- 遵循 openapi 3.0 规范开发

## How to Deploy

1. 配置 Rust 环境 (MSRV 1.64)
2. 设置远程 python 算法 rpc server 地址:
```toml
[common]
model_addr = "xxx:50001"
```

3. 配置好 postgresql 数据库:
```toml
[database]
user_name = "news_recommender"
password = "nekopara"
host = "127.0.0.1"
port = "5432"
db = "news_recommend"
```

4. 安装 [protobuf](https://github.com/protocolbuffers/protobuf)
5. 配置好其他你想配置的东西，然后开启数据库以及远程的 python 算法 rpc server

启动
```shell
cargo run
```

## Deploy in Docker

如果希望整个后端均以 docker 集群的形式部署，首先需要保证 app 容器能够访问 python 算法模块。然后执行以下命令：

```shell
git clone https://github.com/silentEAG/nrs-server && cd nrs-server
cp docker-compose-pro.yml docker-compose.yml
docker compose up -d
```

## Devlopment

需要 Rust 1.64 及以上开发环境，以下命令只会启动 db 容器，rust server 会在物理机上运行。

```shell
git clone https://github.com/silentEAG/nrs-server && cd nrs-server
cp docker-compose-dev.yml docker-compose.yml
docker compose up -d
```

## MSRV
1.64.0