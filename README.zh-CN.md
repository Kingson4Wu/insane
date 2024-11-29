# Insane Web Service

一个使用 Rust、Axum 和 Tokio 构建的现代化 Web 服务。

## 功能特性

- 🚀 基于 Axum 的高性能 Web 框架
- ⚡ Tokio 异步运行时支持
- 📊 内置访问计数器（使用原子计数器实现）
- 🔍 健康检查端点
- 📝 JSON 消息处理
- 📈 请求日志记录

## 技术栈

- Rust 2021 Edition
- Axum 0.7.3 (Web 框架)
- Tokio 1.35.1 (异步运行时)
- Tower 0.4.13 (HTTP 服务组件)
- Serde (JSON 序列化/反序列化)
- Tracing (日志系统)

## API 端点

### 健康检查
```
GET /health
```
返回服务的健康状态和版本信息。

示例响应：
```json
{
    "status": "healthy",
    "version": "0.1.0"
}
```

### 访问计数器
```
GET /visit
```
返回当前的访问次数。

示例响应：
```
Visit count: 1
```

### 消息回显
```
POST /echo
Content-Type: application/json

{
    "content": "Hello, Axum!"
}
```
将接收到的消息内容原样返回。

## 快速开始

1. 克隆项目
```bash
git clone <repository-url>
cd insane
```

2. 构建项目
```bash
cargo build
```

3. 运行服务
```bash
cargo run
```
服务将在 http://127.0.0.1:3000 启动

## API 测试

使用 curl 测试各个端点：

```bash
# 健康检查
curl http://localhost:3000/health

# 访问计数
curl http://localhost:3000/visit

# 消息回显
curl -X POST -H "Content-Type: application/json" \
     -d '{"content":"Hello, Axum!"}' \
     http://localhost:3000/echo
```

## 项目结构

```
insane/
├── src/
│   └── main.rs      # 主程序入口
├── Cargo.toml       # 项目依赖配置
└── README.md        # 项目文档
```

## 开发计划

- [ ] 添加数据库集成
- [ ] 实现用户认证
- [ ] 添加更多 API 端点
- [ ] 添加静态文件服务
- [ ] 实现 WebSocket 支持

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License
