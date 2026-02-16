# API specifications (REST)

## POST /api/auth
x-www-form-urlencoded
{
  email: String,
  password: String,
}

Response:
application/text
HTTP OK
token: uuid

HTTP UNAUTHORIZED (或其他)
fault: String, // 错误原因

## POST /api/add_user
Authorization: Bearer <token> // 必须是 admin 的 token
x-www-form-urlencoded
{
  email: String,
  password: String,
}

Response:
application/json
HTTP OK
{
  id: Uuid,
}

HTTP UNAUTHORIZED (或其他)
fault: String, // 错误原因

GET /api/user/list
Authorization: Bearer <token> // 必须是 admin 的 token

Response
application/json
HTTP OK
[
  {
    id: Uuid,
    email: String,
  },
]

HTTP UNAUTHORIZED (或其他)
fault: String, // 错误原因

DELETE /api/user/delete
Authorization: Bearer <token> // 必须是 admin 的 token
x-www-form-urlencoded
{
  id: Uuid,
  OR
  email: String,
}

Response:
application/text
HTTP OK
deleted

HTTP UNAUTHORIZED (或其他)
fault: String, // 错误原因

POST /api/chat
Authorization: Bearer <token> 
application/json
{
  agent_id: Uuid,
  conversation_id: Uuid,
  content: String,
}

Response:
application/json
HTTP OK
{
  content: String,
  name: String,
  emotion: String,
  favorability: i32,
  (if vip) mind: String,
}

HTTP UNAUTHORIZED (或其他)
{
  content: String,
  name: String,
  emotion: String,
  favorability: i32,
  (if vip) mind: String,
}
