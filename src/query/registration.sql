insert into user1 (name, email, password) values ($1, $2, $3) 
on conflict (email) 
do nothing
-- do update set email = excluded.email
returning id;