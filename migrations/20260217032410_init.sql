-- Add migration script here
-- 启用扩展
create extension if not exists "pgcrypto";

-- =========================
-- users
-- =========================

create table users (
    id uuid primary key default gen_random_uuid(),
    email text unique not null,
    password_hash text not null,
    vip boolean not null default false,
    name text not null default '',
    is_admin boolean not null default false,
    created_at timestamptz not null default now()
);

create index idx_users_email on users(email);

-- =========================
-- agents
-- =========================

create table agents (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,

    name text not null,
    emotion text not null,
    favorability int not null default 0,
    character_design text not null,
    response_requirement text not null,
    character_emotion_split text not null,

    model text not null,
    temperature double precision not null default 1.0,
    max_tokens int,

    created_at timestamptz not null default now()
);

create index idx_agents_user_id on agents(user_id);

-- =========================
-- conversations
-- =========================

create table conversations (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    agent_id uuid not null references agents(id) on delete cascade,

    title text,
    is_archived boolean not null default false,

    -- ⭐ 核心改动：计数器
    message_count int not null default 0,

    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index idx_conversations_user_id on conversations(user_id);
create index idx_conversations_agent_id on conversations(agent_id);
create index idx_conversations_updated_at on conversations(updated_at desc);

-- =========================
-- messages
-- =========================

create table messages (
    id uuid primary key default gen_random_uuid(),
    conversation_id uuid not null references conversations(id) on delete cascade,

    role text not null check (role in ('system','user','assistant','tool')),

    content text,
    name text,
    tool_call_id text,

    tool_calls jsonb,
    reasoning_content text,

    message_index int not null,

    input_tokens int,
    output_tokens int,

    created_at timestamptz not null default now()
);

create unique index idx_messages_conversation_index
on messages(conversation_id, message_index);

create index idx_messages_conversation_created
on messages(conversation_id, created_at);

create index idx_messages_tool_calls
on messages using gin (tool_calls);

-- =========================
-- conversation summaries
-- =========================

create table conversation_summaries (
    conversation_id uuid primary key references conversations(id) on delete cascade,
    summary text not null,
    last_summarized_index int not null,
    updated_at timestamptz not null default now()
);

-- =========================
-- sessions
-- =========================

create table sessions (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    token_hash text not null,
    expires_at timestamptz not null,
    created_at timestamptz not null default now()
);

-- =========================
-- 触发器：更新对话时间
-- =========================

create or replace function update_conversation_timestamp()
returns trigger as $$
begin
    update conversations
    set updated_at = now()
    where id = new.conversation_id;

    return new;
end;
$$ language plpgsql;

create trigger trg_update_conversation_timestamp
after insert on messages
for each row
execute function update_conversation_timestamp();

-- =========================
-- 并发安全的 message_index 分配函数
-- =========================

create or replace function next_message_index(p_conversation_id uuid)
returns int as $$
declare
    new_index int;
begin
    update conversations
    set message_count = message_count + 1
    where id = p_conversation_id
    returning message_count into new_index;

    return new_index;
end;
$$ language plpgsql;
