UPDATE users
SET
    password_hash = '$argon2id$v=19$m=15000,t=2,p=1$TlNzVUsweHRhS2ZVM05XVQ$Y78s3JsxpJ6ot/dDO5+2Qw'
WHERE user_id = 'ddf8994f-d522-4659-8d02-c1d479057be6'
