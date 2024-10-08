-- Add up migration script here
create table theatre_show (
    id serial primary key,
    theatre_id int not null,
    show_time timestamp not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,
    foreign key (theatre_id) references theatre(id)
);