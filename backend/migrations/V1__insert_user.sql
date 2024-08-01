-- V1__insert-user.sql
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM users WHERE username = 'ferris') THEN
        INSERT INTO users (username, password_hash) 
        VALUES ('ferris', '$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw');
    END IF;
END $$;