-- Add migration script here
alter table users
add column vip boolean not null default false,
add column name text not null default '';
