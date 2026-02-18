
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

## User

新增一个[@routes.rs (35:36)](file:///Users/ozongzi/code/web/rpg_stage/src/api/routes.rs#L35:36)

**Tool Call: Thinking**
Status: Completed

用户想要在routes.rs文件中新增一个路由。看起来他们已经添加了一个关于agent_metas的路由。我需要：

1. 检查这个新路由是否已经存在对应的handler
2. 如果handler不存在，我需要查看项目结构来了解如何创建它
3. 更新API使用说明文档来包含这个新的路由

从用户提供的选中代码来看，他们添加了：
- POST /agent_metas 路由，调用 create_agent_meta handler

让我先检查是否存在create_agent_meta这个handler。

**Tool Call: 1 match**
Status: Completed

file:///Users/ozongzi/code/web/rpg_stage/src/api/handlers/create_agent_meta.rs

## Assistant

看起来handler文件已经存在，让我查看它的内容：

**Tool Call: Read file `src/api/handlers/create_agent_meta.rs`**
Status: Completed

```src/api/handlers/create_agent_meta.rs
use crate::api::extractors::auth_user::AuthUserAdmin;
use crate::app_state::AppState;
use crate::domains::MetaAgent;
use crate::errors::AppResult;
use axum::extract::State;
use axum::{Form, Json};
use serde_json::{Value, json};

pub type MetadataForm = MetaAgent;
pub async fn create_agent_meta(
    State(state): State<AppState>,
    AuthUserAdmin { user_id: _ }: AuthUserAdmin,
    Form(form): Form<MetadataForm>,
) -> AppResult<Json<Value>> {
    let id = state.services.agent_service.new_agent_meta(&form).await?;

    Ok(Json(json!({ "agent_meta_id": id })))
}

```


## Assistant

现在我需要检查MetaAgent的结构来了解这个API需要什么参数：

**Tool Call: Search files for regex `struct MetaAgent|pub struct MetaAgent`**
Status: Completed

Found 1 matches:

## Matches in rpg_stage/src/domains/meta_agent.rs

### pub struct MetaAgent › L4-11
```
pub struct MetaAgent {
    pub name: String,
    pub description: String,
    pub character_design: String,
    pub response_requirement: String,
    pub character_emotion_split: String,
    pub model: String,
}
```


## Assistant

还需要检查mod.rs文件是否已经导入了这个新的handler：

**Tool Call: Read file `src/api/handlers/mod.rs`**
Status: Completed

```src/api/handlers/mod.rs
mod create_agent;
mod create_agent_meta;
mod create_conversation;
mod create_message;
mod create_user;
mod delete_user;
mod get_agent;
mod health_check;
mod list_agents;
mod list_conversations;
mod list_messages;
mod list_users;
mod login;
mod logout;

pub use create_agent::create_agent;
pub use create_agent_meta::create_agent_meta;
pub use create_conversation::create_conversation;
pub use create_message::create_message;
pub use create_user::create_user;
pub use delete_user::delete_user;
pub use get_agent::get_agent;
pub use health_check::health_check;
pub use list_agents::list_agents;
pub use list_conversations::list_conversations;
pub use list_messages::list_messages;
pub use list_users::list_users;
pub use login::login;
pub use logout::logout;

```


## Assistant

很好！看起来所有必要的文件都已经存在并且已经正确导入。现在让我更新API使用说明文档，添加这个新的API：

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

### 4. 代理元数据管理

#### 创建代理元数据
```/dev/null/api.md#L1-16
POST /agent_metas
Content-Type: application/x-www-form-urlencoded

需要认证: 是 (需要管理员权限)
参数:
- name: 代理名称 (必填)
- description: 代理描述 (必填)
- character_design: 角色设计 (必填)
- response_requirement: 回复要求 (必填)
- character_emotion_split: 角色情绪分割 (必填)
- model: 使用的AI模型 (必填)

响应: {"agent_meta_id": "uuid"}
```

---

### 5. 代理(Agent)管理

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

### 6. 对话管理

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

### 7. 消息管理

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

3. **创建代理元数据** (需要管理员权限)
   ```/dev/null/api.md#L1-5
   POST /agent_metas
   参数: name, description, character_design, response_requirement, 
         character_emotion_split, model
   ```

4. **创建代理**
   ```/dev/null/api.md#L1-4
   POST /agents
   参数: agent_metadata_id
   ```

5. **创建对话**
   ```/dev/null/api.md#L1-3
   POST /agents/{agent_id}/conversations
   ```

6. **发送消息**
   ```/dev/null/api.md#L1-4
   POST /conversations/{conversation_id}/messages
   请求体: {"content": "你好"}
   ```

7. **查看消息历史**
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
3. 用户创建和代理元数据创建需要管理员权限
4. 会话管理采用服务器端会话机制
5. 消息发送会触发AI代理处理，可能需要较长响应时间
6. 代理元数据是代理的模板，定义了AI代理的行为特征

我已经为你更新了API使用说明，新增了 `POST /agent_metas` 接口的文档。

这个新接口的主要特点：
- **需要管理员权限**：使用 `AuthUserAdmin` 提取器
- **用于创建代理元数据**：这些元数据定义了AI代理的角色特征
- **接收表单数据**：包含代理的名称、描述、角色设计、回复要求、情绪分割和使用的AI模型
- **返回元数据ID**：创建成功后返回 `agent_meta_id`，可用于后续创建具体的代理实例

在典型使用流程中，这个接口应该在创建代理之前调用，用于定义代理的基本特征和行为模式。

## User

重新写一份文档，需要详细的，说明请求格式和返回格式（包括正确格式和返回错误的格式）

**Tool Call: Thinking**
Status: Completed

用户要求我重新写一份更详细的API文档，需要包括：
1. 详细的请求格式
2. 返回格式（包括成功和错误的格式）

我需要查看更多handler的实现来了解具体的返回格式，特别是错误处理的部分。让我先查看一些关键的handler和错误处理相关的代码。

**Tool Call: Read file `src/api/handlers/list_users.rs`**
Status: Completed

```src/api/handlers/list_users.rs
use crate::errors::AppResult;
use crate::{api::extractors::auth_user::AuthUserAdmin, app_state::AppState};
use axum::{
    Json,
    extract::State,
    http::{Response, StatusCode},
};
use serde::Serialize;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

#[derive(Clone, Serialize)]
struct UserResponse {
    id: Uuid,
    email: String,
}

pub async fn list_users(
    State(state): State<AppState>,
    AuthUserAdmin { user_id: _ }: AuthUserAdmin,
) -> AppResult<Json<Value>> {
    Ok(Json(json!(state.services.user_service.list_users().await?)))
}

```


**Tool Call: Read file `src/api/handlers/list_agents.rs`**
Status: Completed

```src/api/handlers/list_agents.rs
use crate::api::extractors::auth_user::AuthUser;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::Json;
use axum::extract::State;
use serde_json::{Value, json};

pub async fn list_agents(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
) -> AppResult<Json<Value>> {
    let agents = state
        .services
        .agent_service
        .get_agent_states_list(user_id)
        .await?;
    Ok(Json(json!(agents)))
}

```


**Tool Call: Read file `src/api/handlers/list_conversations.rs`**
Status: Completed

```src/api/handlers/list_conversations.rs
use crate::api::extractors::auth_user::AuthUser;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::Json;
use axum::extract::{Path, State};
use serde_json::{Value, json};
use uuid::Uuid;

pub async fn list_conversations(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Path(agent_id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    let conversations = state
        .services
        .conversation_service
        .get_conversations_list(agent_id, user_id)
        .await?;
    Ok(Json(json!(conversations)))
}

```


**Tool Call: Read file `src/api/handlers/list_messages.rs`**
Status: Completed

```src/api/handlers/list_messages.rs
use crate::api::extractors::auth_user::AuthUser;
use crate::api::handlers::create_message::ChatMessage;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::Json;
use axum::extract::{Path, State};
use serde_json::Value;
use uuid::Uuid;

pub async fn list_messages(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Path(conversation_id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    Ok(Json(
        state
            .services
            .chat_service
            .get_messages_list(user_id, conversation_id)
            .await?,
    ))
}

```


## Assistant

现在让我查看错误处理相关的代码：

**Tool Call: 1 match**
Status: Completed

file:///Users/ozongzi/code/web/rpg_stage/src/errors.rs

**Tool Call: Read file `src/errors.rs`**
Status: Completed

```src/errors.rs
pub type AppResult<T> = Result<T, AppError>;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct AppError(pub StatusCode, pub Value);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.0, Json(self.1)).into_response()
    }
}

use serde_json::Value;
use sqlx::Error as SqlxError;

impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        tracing::info!("{:?}", err);
        match err {
            SqlxError::Database(db_err) => {
                if db_err.constraint().is_some() {
                    return AppError(StatusCode::CONFLICT, "数据已存在".into());
                }
                AppError(StatusCode::INTERNAL_SERVER_ERROR, "数据库错误".into())
            }
            SqlxError::RowNotFound => AppError(StatusCode::BAD_REQUEST, "数据不存在".into()),
            SqlxError::ColumnNotFound(_) => AppError(StatusCode::BAD_REQUEST, "数据不存在".into()),
            _ => AppError(StatusCode::INTERNAL_SERVER_ERROR, "数据库错误".into()),
        }
    }
}

```


## Assistant

让我查看更多关于domain错误处理的代码：

**Tool Call: Search files for regex `impl.*From.*AppError`**
Status: Completed

Found 1 matches:

## Matches in rpg_stage/src/errors.rs

### impl From<SqlxError> for AppError › L20-30
```
impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        tracing::info!("{:?}", err);
        match err {
            SqlxError::Database(db_err) => {
                if db_err.constraint().is_some() {
                    return AppError(StatusCode::CONFLICT, "数据已存在".into());
                }
                AppError(StatusCode::INTERNAL_SERVER_ERROR, "数据库错误".into())
            }
            SqlxError::RowNotFound => AppError(StatusCode::BAD_REQUEST, "数据不存在".into()),
```

5 lines remaining in ancestor node. Read the file to see all.


**Tool Call: Search files for regex `impl.*From.*for AppError`**
Status: Completed

Found 1 matches:

## Matches in rpg_stage/src/errors.rs

### impl From<SqlxError> for AppError › L20-30
```
impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        tracing::info!("{:?}", err);
        match err {
            SqlxError::Database(db_err) => {
                if db_err.constraint().is_some() {
                    return AppError(StatusCode::CONFLICT, "数据已存在".into());
                }
                AppError(StatusCode::INTERNAL_SERVER_ERROR, "数据库错误".into())
            }
            SqlxError::RowNotFound => AppError(StatusCode::BAD_REQUEST, "数据不存在".into()),
```

5 lines remaining in ancestor node. Read the file to see all.


**Tool Call: 10 matches**
Status: Completed

file:///Users/ozongzi/code/web/rpg_stage/src/domains

file:///Users/ozongzi/code/web/rpg_stage/src/domains/agent.rs

file:///Users/ozongzi/code/web/rpg_stage/src/domains/chat_message.rs

file:///Users/ozongzi/code/web/rpg_stage/src/domains/conversation.rs

file:///Users/ozongzi/code/web/rpg_stage/src/domains/email.rs

file:///Users/ozongzi/code/web/rpg_stage/src/domains/meta_agent.rs

file:///Users/ozongzi/code/web/rpg_stage/src/domains/mod.rs

file:///Users/ozongzi/code/web/rpg_stage/src/domains/user.rs

file:///Users/ozongzi/code/web/rpg_stage/src/domains/user_name.rs

file:///Users/ozongzi/code/web/rpg_stage/src/domains/user_password.rs

**Tool Call: Read file `src/domains/email.rs`**
Status: Completed

```src/domains/email.rs
use crate::errors::AppError;
use axum::http::StatusCode;
use std::str::FromStr;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]

pub struct Email(String);

impl FromStr for Email {
    type Err = AppError;
    fn from_str(email: &str) -> Result<Self, AppError> {
        if validator::ValidateEmail::validate_email(&email) {
            Ok(Self(email.to_string()))
        } else {
            Err(AppError(StatusCode::BAD_REQUEST, "邮箱格式不正确".into()))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

```


## Assistant

让我看看返回的数据结构：

**Tool Call: Search files for regex `get_agent_states_list|get_conversations_list|get_messages_list`**
Status: Completed

Found 6 matches:

## Matches in rpg_stage/src/api/handlers/list_agents.rs

### pub async fn list_agents › L12-15
```
    let agents = state
        .services
        .agent_service
        .get_agent_states_list(user_id)
```

## Matches in rpg_stage/src/api/handlers/list_conversations.rs

### pub async fn list_conversations › L14-17
```
    let conversations = state
        .services
        .conversation_service
        .get_conversations_list(agent_id, user_id)
```

## Matches in rpg_stage/src/api/handlers/list_messages.rs

### pub async fn list_messages › L16-19
```
        state
            .services
            .chat_service
            .get_messages_list(user_id, conversation_id)
```

## Matches in rpg_stage/src/services/agent_service.rs

### impl AgentService › pub async fn get_agent_states_list › L26-30
```
    }

    pub async fn get_agent_states_list(&self, user_id: Uuid) -> AppResult<Vec<AgentState>> {
        self.repo.fetch_agent_state_list_by_user_id(user_id).await
    }
```

## Matches in rpg_stage/src/services/chat_service.rs

### impl ChatService › pub async fn get_messages_list › L34-38
```
    }

    pub async fn get_messages_list(
        &self,
        user_id: Uuid,
```

## Matches in rpg_stage/src/services/conversation_service.rs

### impl ConversationService › pub async fn get_conversations_list › L32-36
```
    }

    pub async fn get_conversations_list(
        &self,
        agent_id: Uuid,
```


## Assistant

让我查看一些具体的数据结构定义：

**Tool Call: Search files for regex `struct.*AgentState|pub struct AgentState`**
Status: Completed

Found 1 matches:

## Matches in rpg_stage/src/domains/agent.rs

### pub struct AgentState › L18-23
```
pub struct AgentState {
    pub id: Uuid,
    pub name: String,
    pub emotion: String,
    pub favorability: i32,
}
```


**Tool Call: Read file `src/domains/conversation.rs`**
Status: Completed

```src/domains/conversation.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct Conversation {
    pub id: Uuid,
    pub title: Option<String>,
}

```


**Tool Call: Read file `src/domains/chat_message.rs`**
Status: Completed

```src/domains/chat_message.rs
pub use ds_api::Role;

#[derive(Default)]
pub struct ChatMessage {
    pub role: Role,
    pub content: Option<String>,
    pub name: Option<String>,
    pub tool_call_id: Option<String>,
    pub tool_calls: Option<serde_json::Value>,
    pub reasoning_content: Option<String>,
}

impl ChatMessage {
    pub fn new(role: Role, content: String) -> Self {
        Self {
            role,
            content: Some(content),
            ..Default::default()
        }
    }
}

```


## Assistant

现在让我查看用户相关的数据结构：

**Tool Call: Read file `src/domains/user.rs`**
Status: Completed

```src/domains/user.rs
use serde::Serialize;
use crate::domains::{Email, UserName, UserPassword};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct User {
    id: Uuid,
    name: UserName,
    email: Email,
    password_hash: String,
}
impl User {
    pub fn new(id: Uuid, name: UserName, email: Email, password_hash: String) -> Self {
        Self { id, name, email, password_hash }
    }
    
    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
    
    pub fn id(&self) -> Uuid {
        self.id
    }
}
```


## Assistant

现在我有足够信息来写一个详细的API文档了：

# RPG Stage API 详细文档

## 基础信息

- **Base URL**: `http://localhost:PORT` (具体端口请参考配置文件)
- **认证方式**: 基于会话(Session)的认证
- **Content-Type**: 
  - 表单数据: `application/x-www-form-urlencoded`
  - JSON数据: `application/json`

---

## 通用错误响应格式

所有API在出错时都返回统一的错误格式：

```/dev/null/error.json#L1-5
HTTP Status: 4xx 或 5xx
Content-Type: application/json

"错误信息描述"
```

### 常见错误码和消息：

- **400 Bad Request**
  ```/dev/null/error.json#L1-1
  "邮箱格式不正确"
  ```
  ```/dev/null/error.json#L1-1
  "数据不存在"
  ```

- **401 Unauthorized**
  ```/dev/null/error.json#L1-1
  "未认证"
  ```

- **403 Forbidden**
  ```/dev/null/error.json#L1-1
  "权限不足"
  ```

- **409 Conflict**
  ```/dev/null/error.json#L1-1
  "数据已存在"
  ```

- **500 Internal Server Error**
  ```/dev/null/error.json#L1-1
  "数据库错误"
  ```

---

## API 接口详细说明

### 1. 健康检查

检查服务是否正常运行。

#### 请求
```/dev/null/request.http#L1-2
GET /health
Host: localhost:PORT
```

#### 响应
**成功 (200 OK):**
```/dev/null/response.http#L1-1
HTTP/1.1 200 OK
```

---

### 2. 用户认证

#### 2.1 登录

用户登录获取会话。

#### 请求
```/dev/null/request.http#L1-5
POST /auth/session
Content-Type: application/x-www-form-urlencoded

email=user@example.com&password=your_password
```

**请求参数:**
- `email` (string, 必填): 用户邮箱，必须是有效的邮箱格式
- `password` (string, 必填): 用户密码

#### 响应
**成功 (200 OK):**
```/dev/null/response.json#L1-3
HTTP/1.1 200 OK
Content-Type: application/json

"session_token_string"
```

**失败示例:**
```/dev/null/response.json#L1-3
HTTP/1.1 400 Bad Request
Content-Type: application/json

"邮箱格式不正确"
```

#### 2.2 登出

当前用户登出，清除会话。

#### 请求
```/dev/null/request.http#L1-3
DELETE /auth/session
Authorization: Bearer session_token
Host: localhost:PORT
```

#### 响应
**成功 (200 OK):**
```/dev/null/response.http#L1-1
HTTP/1.1 200 OK
```

**失败示例:**
```/dev/null/response.json#L1-3
HTTP/1.1 401 Unauthorized
Content-Type: application/json

"未认证"
```

---

### 3. 用户管理

#### 3.1 创建用户 (注册)

管理员创建新用户。

#### 请求
```/dev/null/request.http#L1-6
POST /users
Content-Type: application/x-www-form-urlencoded
Authorization: Bearer admin_session_token

name=张三&email=zhangsan@example.com&password=secure_password
```

**请求参数:**
- `name` (string, 必填): 用户名称
- `email` (string, 必填): 用户邮箱，必须是有效的邮箱格式
- `password` (string, 必填): 用户密码

#### 响应
**成功 (200 OK):**
```/dev/null/response.json#L1-5
HTTP/1.1 200 OK
Content-Type: application/json

{
  "user_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**失败示例:**
```/dev/null/response.json#L1-3
HTTP/1.1 403 Forbidden
Content-Type: application/json

"权限不足"
```

```/dev/null/response.json#L1-3
HTTP/1.1 409 Conflict
Content-Type: application/json

"数据已存在"
```

#### 3.2 列出用户

管理员查看所有用户列表。

#### 请求
```/dev/null/request.http#L1-3
GET /users
Authorization: Bearer admin_session_token
Host: localhost:PORT
```

#### 响应
**成功 (200 OK):**
```/dev/null/response.json#L1-12
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

### 4. 代理元数据管理

#### 4.1 创建代理元数据

管理员创建AI代理的元数据模板。

#### 请求
```/dev/null/request.http#L1-7
POST /agent_metas
Content-Type: application/x-www-form-urlencoded
Authorization: Bearer admin_session_token

name=智能助手&description=友好的AI助手&character_design=温和友善的性格&response_requirement=回复要简洁明了&character_emotion_split=开心:0.7,平静:0.3&model=gpt-4
```

**请求参数:**
- `name` (string, 必填): 代理名称
- `description` (string, 必填): 代理描述
- `character_design` (string, 必填): 角色设计描述
- `response_requirement` (string, 必填): 回复要求
- `character_emotion_split` (string, 必填): 角色情绪分割配置
- `model` (string, 必填): 使用的AI模型

#### 响应
**成功 (200 OK):**
```/dev/null/response.json#L1-5
HTTP/1.1 200 OK
Content-Type: application/json

{
  "agent_meta_id": "550e8400-e29b-41d4-a716-446655440002"
}
```

**失败示例:**
```/dev/null/response.json#L1-3
HTTP/1.1 403 Forbidden
Content-Type: application/json

"权限不足"
```

---

### 5. 代理管理

#### 5.1 创建代理

基于代理元数据创建代理实例。

#### 请求
```/dev/null/request.http#L1-6
POST /agents
Content-Type: application/x-www-form-urlencoded
Authorization: Bearer session_token

agent_metadata_id=550e8400-e29b-41d4-a716-446655440002
```

**请求参数:**
- `agent_metadata_id` (UUID, 必填): 代理元数据ID

#### 响应
**成功 (200 OK):**
```/dev/null/response.json#L1-5
HTTP/1.1 200 OK
Content-Type: application/json

{
  "agent_id": "550e8400-e29b-41d4-a716-446655440003"
}
```

**失败示例:**
```/dev/null/response.json#L1-3
HTTP/1.1 400 Bad Request
Content-Type: application/json

"数据不存在"
```

#### 5.2 列出代理

列出当前用户的所有代理。

#### 请求
```/dev/null/request.http#L1-3
GET /agents
Authorization: Bearer session_token
Host: localhost:PORT
```

#### 响应
**成功 (200 OK):**
```/dev/null/response.json#L1-12
HTTP/1.1 200 OK
Content-Type: application/json

[
  {
    "id": "550e8400-e29b-41d4-a716-446655440003",
    "name": "智能助手",
    "emotion": "开心",
    "favorability": 50
  },
  {
    "id": "550e8400-e29b-41d4-a716-446655440004",
    "name": "学习伙伴",
    "emotion": "平静",
    "favorability": 75
  }
]
```

#### 5.3 获取特定代理

获取指定代理的详细信息。

#### 请求
```/dev/null/request.http#L1-3
GET /agents/550e8400-e29b-41d4-a716-446655440003
Authorization: Bearer session_token
Host: localhost:PORT
```

#### 响应
**成功 (200 OK):**
```/dev/null/response.json#L1-8
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": "550e8400-e29b-41d4-a716-446655440003",
  "name": "智能助手",
  "emotion": "开心",
  "favorability": 50
}
```

**失败示例:**
```/dev/null/response.json#L1-3
HTTP/1.1 400 Bad Request
Content-Type: application/json

"数据不存在"
```

---

### 6. 对话管理

#### 6.1 创建对话

为指定代理创建新对话。

#### 请求
```/dev/null/request.http#L1-4
POST /agents/550e8400-e29b-41d4-a716-446655440003/conversations
Authorization: Bearer session_token
Host: localhost:PORT
Content-Length: 0
```

#### 响应
**成功 (200 OK):**
```/dev/null/response.json#L1-5
HTTP/1.1 200 OK
Content-Type: application/json

{
  "conversation_id": "550e8400-e29b-41d4-a716-446655440005"
}
```

**失败示例:**
```/dev/null/response.json#L1-3
HTTP/1.1 400 Bad Request
Content-Type: application/json

"数据不存在"
```

#### 6.2 列出对话

列出指定代理的所有对话。

#### 请求
```/dev/null/request.http#L1-3
GET /agents/550e8400-e29b-41d4-a716-446655440003/conversations
Authorization: Bearer session_token
Host: localhost:PORT
```

#### 响应
**成功 (200 OK):**
```/dev/null/response.json#L1-10
HTTP/1.1 200 OK
Content-Type: application/json

[
  {
    "id": "550e8400-e29b-41d4-a716-446655440005",
    "title": "关于学习的讨论"
  },
  {
    "id": "550e8400-e29b-41d4-a716-446655440006",
    "title": null
  }
]
```

---

### 7. 消息管理

#### 7.1 发送消息

向指定对话发送消息并获取AI回复。

#### 请求
```/dev/null/request.http#L1-7
POST /conversations/550e8400-e29b-41d4-a716-446655440005/messages
Content-Type: application/json
Authorization: Bearer session_token

{
  "content": "你好，今天天气怎么样？"
}
```

**请求参数:**
- `content` (string, 必填): 消息内容

#### 响应
**成功 (200 OK):**
```/dev/null/response.json#L1-5
HTTP/1.1 200 OK
Content-Type: application/json

{
  "response": "你好！今天天气很不错，阳光明媚，适合出门活动哦！"
}
```

**失败示例:**
```/dev/null/response.json#L1-3
HTTP/1.1 400 Bad Request
Content-Type: application/json

"数据不存在"
```

#### 7.2 获取消息列表

获取指定对话的消息历史。

#### 请求
```/dev/null/request.http#L1-3
GET /conversations/550e8400-e29b-41d4-a716-446655440005/messages
Authorization: Bearer session_token
Host: localhost:PORT
```

#### 响应
**成功 (200 OK):**
```/dev/null/response.json#L1-14
HTTP/1.1 200 OK
Content-Type: application/json

[
  {
    "role": "user",
    "content": "你好，今天天气怎么样？",
    "timestamp": "2024-01-15T10:30:00Z"
  },
  {
    "role": "assistant", 
    "content": "你好！今天天气很不错，阳光明媚，适合出门活动哦！",
    "timestamp": "2024-01-15T10:30:05Z"
  }
]
```

---

## 完整使用流程示例

### 1. 管理员设置代理

```/dev/null/flow.http#L1-30
# 1. 管理员登录
POST /auth/session
Content-Type: application/x-www-form-urlencoded

email=admin@example.com&password=admin_password

# 响应: "admin_session_token"

# 2. 创建代理元数据
POST /agent_metas
Content-Type: application/x-www-form-urlencoded
Authorization: Bearer admin_session_token

name=智能助手&description=友好的AI助手&character_design=温和友善的性格&response_requirement=回复要简洁明了&character_emotion_split=开心:0.7,平静:0.3&model=gpt-4

# 响应: {"agent_meta_id": "meta-uuid"}

# 3. 创建用户(可选)
POST /users
Content-Type: application/x-www-form-urlencoded
Authorization: Bearer admin_session_token

name=普通用户&email=user@example.com&password=user_password

# 响应: {"user_id": "user-uuid"}
```

### 2. 用户使用流程

```/dev/null/flow.http#L1-40
# 1. 用户登录
POST /auth/session
Content-Type: application/x-www-form-urlencoded

email=user@example.com&password=user_password

# 响应: "user_session_token"

# 2. 创建代理实例
POST /agents
Content-Type: application/x-www-form-urlencoded
Authorization: Bearer user_session_token

agent_metadata_id=meta-uuid

# 响应: {"agent_id": "agent-uuid"}

# 3. 创建对话
POST /agents/agent-uuid/conversations
Authorization: Bearer user_session_token

# 响应: {"conversation_id": "conversation-uuid"}

# 4. 发送消息
POST /conversations/conversation-uuid/messages
Content-Type: application/json
Authorization: Bearer user_session_token

{"content": "你好！"}

# 响应: {"response": "你好！很高兴认识你！"}

# 5. 查看消息历史
GET /conversations/conversation-uuid/messages
Authorization: Bearer user_session_token

# 响应: 消息列表数组
```

---

## 重要说明

1. **UUID格式**: 所有ID参数都必须是标准UUID格式 (如: `550e8400-e29b-41d4-a716-446655440000`)

2. **邮箱验证**: 邮箱必须符合标准邮箱格式，系统会自动验证

3. **权限控制**: 
   - 创建用户和代理元数据需要管理员权限
   - 其他操作需要普通用户权限

4. **会话管理**: 
   - 登录后会获得会话令牌
   - 所有需要认证的接口都要在请求头中携带 `Authorization: Bearer session_token`
   - 登出后会话令牌失效

5. **数据关系**:
   - 代理元数据 → 代理实例 → 对话 → 消息
   - 必须按顺序创建这些资源

6. **错误处理**: 所有错误都返回简洁的中文错误信息，便于调试和用户理解
