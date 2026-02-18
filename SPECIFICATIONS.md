# RPG Stage API 使用说明

本文档描述了已实现的API接口及其使用方法。

## 基础信息

- **Base URL**: `http://localhost:PORT` (具体端口请参考配置文件)
- **Content-Type**: `application/x-www-form-urlencoded` (表单数据) 或 `application/json` (JSON数据)

## 认证说明

大部分API需要用户认证。认证方式采用会话(session)机制。用户需要先登录获取会话，然后在后续请求中携带会话信息。

## API 接口列表

### 1. 健康检查

检查服务是否正常运行。

```/dev/null/api.md#L1-3
GET /health

响应: 200 OK
```

---

### 2. 用户认证

#### 登录
```/dev/null/api.md#L1-10
POST /auth/session
Content-Type: application/x-www-form-urlencoded

参数:
- email: 用户邮箱 (必填)
- password: 用户密码 (必填)

响应: 会话令牌字符串
```

#### 登出
```/dev/null/api.md#L1-5
DELETE /auth/session

需要认证: 是
响应: 200 OK
```

---

### 3. 用户管理

#### 创建用户(注册)
```/dev/null/api.md#L1-12
POST /users
Content-Type: application/x-www-form-urlencoded

需要认证: 是 (需要管理员权限)
参数:
- name: 用户名称 (必填)
- email: 用户邮箱 (必填)
- password: 用户密码 (必填)

响应: {"user_id": "uuid"}
```

#### 列出用户
```/dev/null/api.md#L1-5
GET /users

需要认证: 是 (需要管理员权限)
响应: 用户列表
```

---

### 4. 代理(Agent)管理

#### 创建代理
```/dev/null/api.md#L1-10
POST /agents
Content-Type: application/x-www-form-urlencoded

需要认证: 是
参数:
- agent_metadata_id: 代理元数据ID (UUID格式, 必填)

响应: {"agent_id": "uuid"}
```

#### 列出代理
```/dev/null/api.md#L1-5
GET /agents

需要认证: 是
响应: 代理列表
```

#### 获取特定代理
```/dev/null/api.md#L1-6
GET /agents/{id}

需要认证: 是
路径参数:
- id: 代理ID (UUID格式)

响应: 代理详细信息
```

---

### 5. 对话管理

#### 创建对话
```/dev/null/api.md#L1-7
POST /agents/{agent_id}/conversations

需要认证: 是
路径参数:
- agent_id: 代理ID (UUID格式)

响应: {"conversation_id": "uuid"}
```

#### 列出对话
```/dev/null/api.md#L1-6
GET /agents/{agent_id}/conversations

需要认证: 是
路径参数:
- agent_id: 代理ID (UUID格式)

响应: 对话列表
```

---

### 6. 消息管理

#### 发送消息
```/dev/null/api.md#L1-11
POST /conversations/{id}/messages
Content-Type: application/json

需要认证: 是
路径参数:
- id: 对话ID (UUID格式)

请求体:
{"content": "消息内容"}

响应: AI回复的消息内容
```

#### 获取消息列表
```/dev/null/api.md#L1-6
GET /conversations/{id}/messages

需要认证: 是
路径参数:
- id: 对话ID (UUID格式)

响应: 消息历史列表
```

---

## 使用流程示例

### 典型的使用流程：

1. **用户注册** (需要管理员权限)
   ```/dev/null/api.md#L1-4
   POST /users
   参数: name, email, password
   ```

2. **用户登录**
   ```/dev/null/api.md#L1-4
   POST /auth/session  
   参数: email, password
   ```

3. **创建代理**
   ```/dev/null/api.md#L1-4
   POST /agents
   参数: agent_metadata_id
   ```

4. **创建对话**
   ```/dev/null/api.md#L1-3
   POST /agents/{agent_id}/conversations
   ```

5. **发送消息**
   ```/dev/null/api.md#L1-4
   POST /conversations/{conversation_id}/messages
   请求体: {"content": "你好"}
   ```

6. **查看消息历史**
   ```/dev/null/api.md#L1-3
   GET /conversations/{conversation_id}/messages
   ```

---

## 错误处理

API使用标准HTTP状态码：

- `200` - 成功
- `400` - 请求参数错误  
- `401` - 未认证
- `403` - 权限不足
- `404` - 资源不存在
- `500` - 服务器内部错误

错误响应通常包含详细的错误信息以帮助调试。

---

## 注意事项

1. 所有UUID参数必须是有效的UUID格式
2. 邮箱地址必须是有效的邮箱格式
3. 用户创建需要管理员权限
4. 会话管理采用服务器端会话机制
5. 消息发送会触发AI代理处理，可能需要较长响应时间
