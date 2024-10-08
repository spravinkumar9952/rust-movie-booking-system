-- Add up migration script here
create table show_booking (
    id serial primary key,
    show_id int not null,
    user_id int not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,
    seat_number int not null,
    "status" varchar(255) not null,
    price float not null,
    qr_code varchar(255) not null,
    foreign key (show_id) references theatre_show(id),
    foreign key (user_id) references users(id)
);