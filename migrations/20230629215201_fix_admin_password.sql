UPDATE users
SET
    password_hash = '$argon2id$v=19$m=15000,t=2,p=1$YWRtaW5zYWx0$gAzUoG+iAIVouuQKchxP0A'
WHERE
    user_id = 'ddf8994f-d522-4659-8d02-c1d479057be6';
