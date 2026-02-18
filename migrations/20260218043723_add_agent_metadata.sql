-- Add migration script here
create table if not exists agent_metadata
(
    id                      uuid primary key     default gen_random_uuid(),
    name                    text        not null,
    description             text        not null,
    created_at              timestamptz not null default now(),

    character_design        text        not null,
    response_requirement    text        not null,
    character_emotion_split text        not null,
    model                   text        not null
)