-- Add migration script here

-- password is "everythinghastostartsomewhere"
INSERT INTO users (id, name, email, password_hash, role)
VALUES (
    'ddf8994f-d522-4659-8d02-c1d479057be7',
    'Test',
    'reviewer@reviewer.com',
    '$argon2id$v=19$m=15000,t=2,p=1$OEx/rcq+3ts//WUDzGNl2g$Am8UFBA4w5NJEmAtquGvBmAlu92q/VQcaoL5AyJPfc8',
    'Reviewer'
);
