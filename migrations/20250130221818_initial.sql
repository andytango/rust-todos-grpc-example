create table todos
(
  todo_id     text primary key     default gen_random_uuid(),
  title       text        not null,
  description text        not null,
  completed   boolean     not null default false,
  created_at  timestamptz not null default now(),
  updated_at  timestamptz not null default now()
);

create function trigger_update_timestamp()
  returns trigger as
$$
begin
  new.updated_at = now();
  return new;
end;
$$ language plpgsql;

create trigger update_timestamp
  before update
  on todos
  for each row
execute procedure trigger_update_timestamp();