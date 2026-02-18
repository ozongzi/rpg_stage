-- Add migration script here
-- =========================
-- agents_memory
-- =========================

create table if not exists agent_memories (
    id uuid primary key default gen_random_uuid(),

    -- 所属智能体
    agent_id uuid not null references agents(id) on delete cascade,

    -- 记忆内容
    content text not null,

    created_at timestamptz not null default now()
);

create index idx_agent_memories_agent_id on agent_memories(agent_id);
-- create index idx_agent_memories_conversation_id on agent_memories(conversation_id);
-- create index idx_agent_memories_memory_type on agent_memories(memory_type);
