DELETE FROM users WHERE user_id = 'ddf8994f-d522-4659-8d02-c1d479057be6';

INSERT INTO
    users (
        user_id,
        username,
        password_hash
    )
VALUES (
        'ddf8994f-d522-4659-8d02-c1d479057be6',
        'admin',
        '$argon2id$v=19$m=15000,t=2,p=1$TvzfQtn3awVPDA8BYkd81Q$bbNDiRLQIrZXZlBMTAlVnxBy/22gmuIk0YRl0n8H+k4'
    );
