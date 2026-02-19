# RPG Stage API 详细文档

## 基础信息

- **Base URL**: ""
- **认证方式**: 基于 Session 的认证，登录后将 Session Token 放入请求 Cookie 或 Header
- **Content-Type**:
  - 表单数据接口: `application/x-www-form-urlencoded`
  - JSON 数据接口: `application/json`

---

## 权限等级说明

| 等级 | 说明 |
|------|------|
| 无需认证 | 任何人均可访问 |
| 普通用户 | 需要登录后的有效 Session |
| 管理员 | 需要登录后的有效 Admin Session |

---

## 通用错误响应格式

所有接口在出错时均返回统一格式：

```
HTTP/1.1 4xx / 5xx
Content-Type: application/json

"错误信息描述"
```

### 常见错误码

| HTTP 状态码 | 含义 | 错误信息示例 |
|-------------|------|-------------|
| 400 | 请求参数错误 | `"邮箱格式不正确"` / `"数据不存在"` |
| 401 | 未认证 | `"未认证"` |
| 403 | 权限不足 | `"权限不足"` |
| 409 | 数据冲突 | `"数据已存在"` |
| 500 | 服务器内部错误 | `"数据库错误"` |

---

## 接口列表总览

| 方法 | 路径 | 描述 | 权限 |
|------|------|------|------|
| GET | `/health` | 健康检查 | 无需认证 |
| POST | `/auth/session` | 登录 | 无需认证 |
| DELETE | `/auth/session` | 登出 | 普通用户 |
| POST | `/users` | 创建用户 | 管理员 |
| GET | `/users` | 列出所有用户 | 管理员 |
| GET | `/users/me` | 获取当前用户信息 | 普通用户 |
| PATCH | `/users/me` | 修改当前用户信息 | 普通用户 |
| GET | `/users/{id}` | 获取指定用户信息 | 普通用户 |
| PATCH | `/users/{id}` | 修改指定用户信息 | 管理员 |
| DELETE | `/users/{id}` | 删除指定用户 | 普通用户 |
| POST | `/agent_metas` | 创建代理元数据 | 管理员 |
| GET | `/agent_metas` | 列出所有代理元数据 | 普通用户 |
| POST | `/agents` | 创建代理实例 | 普通用户 |
| GET | `/agents` | 列出当前用户的代理 | 普通用户 |
| GET | `/agents/{id}` | 获取指定代理 | 普通用户 |
| DELETE | `/agents/{id}` | 删除指定代理 | 普通用户 |
| POST | `/agents/{agent_id}/conversations` | 创建对话 | 普通用户 |
| GET | `/agents/{agent_id}/conversations` | 列出代理的对话 | 普通用户 |
| GET | `/agents/{agent_id}/conversations/{id}` | 获取指定对话 | 普通用户 |
| DELETE | `/agents/{agent_id}/conversations/{id}` | 删除指定对话 | 普通用户 |
| POST | `/conversations/{id}/messages` | 发送消息 | 普通用户 |
| GET | `/conversations/{id}/messages` | 获取消息历史 | 普通用户 |
| GET | `/admin/sessions` | 列出所有会话 | 管理员 |
| DELETE | `/admin/sessions/{id}` | 强制登出指定会话 | 管理员 |

---

## 接口详细说明

---

### 1. 健康检查

**GET** `/health`

权限：无需认证

#### 请求

```
GET /health
```

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
```

---

### 2. 认证

---

#### 2.1 登录

**POST** `/auth/session`

权限：无需认证

#### 请求

```
POST /auth/session
Content-Type: application/x-www-form-urlencoded

email=user%40example.com&password=your_password
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| email | string | 是 | 邮箱，须为合法邮箱格式 |
| password | string | 是 | 密码 |

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

"session_token_string"
```

**失败示例**
```
HTTP/1.1 400 Bad Request
Content-Type: application/json

"邮箱格式不正确"
```

---

#### 2.2 登出

**DELETE** `/auth/session`

权限：普通用户

#### 请求

```
DELETE /auth/session
Authorization: Bearer <session_token>
```

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
```

**失败示例**
```
HTTP/1.1 401 Unauthorized
Content-Type: application/json

"未认证"
```

---

### 3. 用户管理

---

#### 3.1 创建用户

**POST** `/users`

权限：管理员

#### 请求

```
POST /users
Content-Type: application/x-www-form-urlencoded
Authorization: Bearer <admin_session_token>

name=张三&email=zhangsan%40example.com&password=secure_password
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| name | string | 是 | 用户名 |
| email | string | 是 | 邮箱，须为合法邮箱格式 |
| password | string | 是 | 密码 |

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

{
  "user_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**失败示例**
```
HTTP/1.1 403 Forbidden
Content-Type: application/json

"权限不足"
```
```
HTTP/1.1 409 Conflict
Content-Type: application/json

"数据已存在"
```

---

#### 3.2 列出所有用户

**GET** `/users`

权限：管理员

#### 请求

```
GET /users
Authorization: Bearer <admin_session_token>
```

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "张三",
    "email": "zhangsan@example.com"
  },
  {
    "id": "550e8400-e29b-41d4-a716-446655440001",
    "name": "李四",
    "email": "lisi@example.com"
  }
]
```

---

#### 3.3 获取当前用户信息

**GET** `/users/me`

权限：普通用户

#### 请求

```
GET /users/me
Authorization: Bearer <session_token>
```

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "张三",
  "email": "zhangsan@example.com"
}
```

---

#### 3.4 修改当前用户信息

**PATCH** `/users/me`

权限：普通用户。修改任何字段前必须提供当前密码 `old_password` 进行验证，其余字段均为可选。

#### 请求

```
PATCH /users/me
Content-Type: application/x-www-form-urlencoded
Authorization: Bearer <session_token>

old_password=current_password&name=新名字&email=new%40example.com&password=new_password
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| old_password | string | 是 | 当前密码，用于身份验证 |
| name | string | 否 | 新用户名 |
| email | string | 否 | 新邮箱，须为合法邮箱格式 |
| password | string | 否 | 新密码 |

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "新名字",
  "email": "new@example.com"
}
```

**失败示例**
```
HTTP/1.1 400 Bad Request
Content-Type: application/json

"邮箱格式不正确"
```

---

#### 3.5 获取指定用户信息

**GET** `/users/{id}`

权限：普通用户

#### 请求

```
GET /users/550e8400-e29b-41d4-a716-446655440000
Authorization: Bearer <session_token>
```

| 路径参数 | 类型 | 说明 |
|----------|------|------|
| id | UUID | 目标用户的 ID |

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "张三",
  "email": "zhangsan@example.com"
}
```

**失败示例**
```
HTTP/1.1 400 Bad Request
Content-Type: application/json

"数据不存在"
```

---

#### 3.6 修改指定用户信息

**PATCH** `/users/{id}`

权限：管理员。所有字段均为可选，至少传一个。

#### 请求

```
PATCH /users/550e8400-e29b-41d4-a716-446655440000
Content-Type: application/x-www-form-urlencoded
Authorization: Bearer <admin_session_token>

name=新名字&email=new%40example.com&password=new_password
```

| 路径参数 | 类型 | 说明 |
|----------|------|------|
| id | UUID | 目标用户的 ID |

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| name | string | 否 | 新用户名 |
| email | string | 否 | 新邮箱，须为合法邮箱格式 |
| password | string | 否 | 新密码 |

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "新名字",
  "email": "new@example.com"
}
```

---

#### 3.7 删除指定用户

**DELETE** `/users/{id}`

权限：普通用户

#### 请求

```
DELETE /users/550e8400-e29b-41d4-a716-446655440000
Authorization: Bearer <session_token>
```

| 路径参数 | 类型 | 说明 |
|----------|------|------|
| id | UUID | 目标用户的 ID |

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
```

**失败示例**
```
HTTP/1.1 400 Bad Request
Content-Type: application/json

"数据不存在"
```

---

### 4. 代理元数据管理（Agent Metadata）

代理元数据是 AI 角色的模板配置，定义角色性格、指令和使用的模型。管理员创建，普通用户只读。

---

#### 4.1 创建代理元数据

**POST** `/agent_metas`

权限：管理员

#### 请求

```
POST /agent_metas
Content-Type: application/x-www-form-urlencoded
Authorization: Bearer <admin_session_token>

name=小助手&description=温柔体贴的AI伙伴&character_design=性格温和，善解人意&response_requirement=回复简洁，语气亲切&character_emotion_split=开心:0.6,平静:0.3,难过:0.1&model=deepseek-chat
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| name | string | 是 | 代理名称 |
| description | string | 是 | 代理描述 |
| character_design | string | 是 | 角色性格与背景设定 |
| response_requirement | string | 是 | 回复风格与要求 |
| character_emotion_split | string | 是 | 情绪权重配置 |
| model | string | 是 | 使用的 AI 模型标识符 |

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

{
  "agent_meta_id": "550e8400-e29b-41d4-a716-446655440002"
}
```

**失败示例**
```
HTTP/1.1 403 Forbidden
Content-Type: application/json

"权限不足"
```

---

#### 4.2 列出所有代理元数据

**GET** `/agent_metas`

权限：普通用户

#### 请求

```
GET /agent_metas
Authorization: Bearer <session_token>
```

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

[
  {
    "id": "550e8400-e29b-41d4-a716-446655440002",
    "name": "小助手",
    "description": "温柔体贴的AI伙伴"
  },
  {
    "id": "550e8400-e29b-41d4-a716-446655440003",
    "name": "学习导师",
    "description": "专注学业辅导的AI老师"
  }
]
```

---

### 5. 代理管理（Agent）

代理是用户基于元数据创建的 AI 角色实例，拥有独立的情绪状态和好感度。

---

#### 5.1 创建代理实例

**POST** `/agents`

权限：普通用户

#### 请求

```
POST /agents
Content-Type: application/x-www-form-urlencoded
Authorization: Bearer <session_token>

agent_metadata_id=550e8400-e29b-41d4-a716-446655440002
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| agent_metadata_id | UUID | 是 | 代理元数据 ID |

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

{
  "agent_id": "550e8400-e29b-41d4-a716-446655440010"
}
```

**失败示例**
```
HTTP/1.1 400 Bad Request
Content-Type: application/json

"数据不存在"
```

---

#### 5.2 列出当前用户的代理

**GET** `/agents`

权限：普通用户

#### 请求

```
GET /agents
Authorization: Bearer <session_token>
```

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

[
  {
    "id": "550e8400-e29b-41d4-a716-446655440010",
    "name": "小助手",
    "emotion": "开心",
    "favorability": 50
  },
  {
    "id": "550e8400-e29b-41d4-a716-446655440011",
    "name": "学习导师",
    "emotion": "平静",
    "favorability": 30
  }
]
```

---

#### 5.3 获取指定代理

**GET** `/agents/{id}`

权限：普通用户

#### 请求

```
GET /agents/550e8400-e29b-41d4-a716-446655440010
Authorization: Bearer <session_token>
```

| 路径参数 | 类型 | 说明 |
|----------|------|------|
| id | UUID | 代理 ID |

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": "550e8400-e29b-41d4-a716-446655440010",
  "name": "小助手",
  "emotion": "开心",
  "favorability": 50
}
```

**失败示例**
```
HTTP/1.1 400 Bad Request
Content-Type: application/json

"数据不存在"
```

---

#### 5.4 删除指定代理

**DELETE** `/agents/{id}`

权限：普通用户（仅可删除自己的代理）

#### 请求

```
DELETE /agents/550e8400-e29b-41d4-a716-446655440010
Authorization: Bearer <session_token>
```

| 路径参数 | 类型 | 说明 |
|----------|------|------|
| id | UUID | 代理 ID |

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

{
  "agent_id": "550e8400-e29b-41d4-a716-446655440010"
}
```

**失败示例**
```
HTTP/1.1 400 Bad Request
Content-Type: application/json

"数据不存在"
```

---

### 6. 对话管理（Conversations）

---

#### 6.1 创建对话

**POST** `/agents/{agent_id}/conversations`

权限：普通用户

#### 请求

```
POST /agents/550e8400-e29b-41d4-a716-446655440010/conversations
Authorization: Bearer <session_token>
```

| 路径参数 | 类型 | 说明 |
|----------|------|------|
| agent_id | UUID | 代理 ID |

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

{
  "conversation_id": "550e8400-e29b-41d4-a716-446655440020"
}
```

---

#### 6.2 列出代理的所有对话

**GET** `/agents/{agent_id}/conversations`

权限：普通用户

#### 请求

```
GET /agents/550e8400-e29b-41d4-a716-446655440010/conversations
Authorization: Bearer <session_token>
```

| 路径参数 | 类型 | 说明 |
|----------|------|------|
| agent_id | UUID | 代理 ID |

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

[
  {
    "id": "550e8400-e29b-41d4-a716-446655440020",
    "title": "关于学习计划的讨论"
  },
  {
    "id": "550e8400-e29b-41d4-a716-446655440021",
    "title": null
  }
]
```

---

#### 6.3 获取指定对话

**GET** `/agents/{agent_id}/conversations/{id}`

权限：普通用户

#### 请求

```
GET /agents/550e8400-e29b-41d4-a716-446655440010/conversations/550e8400-e29b-41d4-a716-446655440020
Authorization: Bearer <session_token>
```

| 路径参数 | 类型 | 说明 |
|----------|------|------|
| agent_id | UUID | 代理 ID |
| id | UUID | 对话 ID |

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": "550e8400-e29b-41d4-a716-446655440020",
  "title": "关于学习计划的讨论"
}
```

**失败示例**
```
HTTP/1.1 400 Bad Request
Content-Type: application/json

"数据不存在"
```

---

#### 6.4 删除指定对话

**DELETE** `/agents/{agent_id}/conversations/{id}`

权限：普通用户（仅可删除自己的对话）

#### 请求

```
DELETE /agents/550e8400-e29b-41d4-a716-446655440010/conversations/550e8400-e29b-41d4-a716-446655440020
Authorization: Bearer <session_token>
```

| 路径参数 | 类型 | 说明 |
|----------|------|------|
| agent_id | UUID | 代理 ID |
| id | UUID | 对话 ID |

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
```

**失败示例**
```
HTTP/1.1 400 Bad Request
Content-Type: application/json

"数据不存在"
```

---

### 7. 消息管理（Messages）

---

#### 7.1 发送消息

**POST** `/conversations/{id}/messages`

权限：普通用户。发送后将触发 AI 处理并同步返回 AI 回复，响应时间取决于 AI 模型。

#### 请求

```
POST /conversations/550e8400-e29b-41d4-a716-446655440020/messages
Content-Type: application/json
Authorization: Bearer <session_token>

{
  "content": "你好"
}
```

| 路径参数 | 类型 | 说明 |
|----------|------|------|
| id | UUID | 对话 ID |

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| content | string | 是 | 用户发送的消息内容 |

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

{
	"content": "你好啊，博士！今天天气不错，适合检查设备或者聊聊天。有什么需要帮忙的吗？",
	"emotion": "友好且热情",
	"favorability": 25,
	"name": "白铁",
	(可选) "mind": "博士和我打招呼了，好激动"
}
```

**失败示例**
```
HTTP/1.1 400 Bad Request
Content-Type: application/json

"数据不存在"
```

---

#### 7.2 获取消息历史

**GET** `/conversations/{id}/messages`

权限：普通用户

#### 请求

```
GET /conversations/550e8400-e29b-41d4-a716-446655440020/messages
Authorization: Bearer <session_token>
```

| 路径参数 | 类型 | 说明 |
|----------|------|------|
| id | UUID | 对话 ID |

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

[
  {
    "role": "user",
    "content": "你好，今天我们来讨论一下学习计划吧"
  },
  {
    "role": "assistant",
    "content": "好的！制定一个合理的学习计划非常重要……"
  }
]
```

**失败示例**
```
HTTP/1.1 400 Bad Request
Content-Type: application/json

"数据不存在"
```

---

### 8. 管理员（Admin）

---

#### 8.1 列出所有会话

**GET** `/admin/sessions`

权限：管理员

#### 请求

```
GET /admin/sessions
Authorization: Bearer <admin_session_token>
```

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
Content-Type: application/json

[
  {
    "id": "550e8400-e29b-41d4-a716-446655440030",
    "user_id": "550e8400-e29b-41d4-a716-446655440000"
  },
  {
    "id": "550e8400-e29b-41d4-a716-446655440031",
    "user_id": "550e8400-e29b-41d4-a716-446655440001"
  }
]
```

---

#### 8.2 强制登出指定会话

**DELETE** `/admin/sessions/{id}`

权限：管理员

#### 请求

```
DELETE /admin/sessions/550e8400-e29b-41d4-a716-446655440030
Authorization: Bearer <admin_session_token>
```

| 路径参数 | 类型 | 说明 |
|----------|------|------|
| id | UUID | 要强制登出的 Session ID |

#### 响应

**成功 200**
```
HTTP/1.1 200 OK
```

**失败示例**
```
HTTP/1.1 400 Bad Request
Content-Type: application/json

"数据不存在"
```

---

## 注意事项

1. **UUID 格式**: 所有 ID 参数须为标准 UUID 格式，如 `550e8400-e29b-41d4-a716-446655440000`
2. **邮箱验证**: 注册/修改邮箱时，系统会自动校验邮箱格式合法性
3. **密码修改**: 普通用户修改自身信息时，必须提供 `old_password` 进行身份验证；管理员修改他人信息无需此限制
4. **数据隔离**: 用户只能操作自己创建的代理、对话和消息
5. **AI 响应延迟**: 发送消息接口会等待 AI 返回后才响应，请适当设置请求超时时间
6. **静态资源**: 前端静态文件由服务器直接托管，未匹配路由将返回 `client/dist/index.html`
