create table task_sources (
    id int primary key,
    source_name varchar null,
    api_key varchar not null,
    url varchar not null
)
