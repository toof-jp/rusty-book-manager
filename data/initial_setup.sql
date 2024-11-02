INSERT INTO
    roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users (name, email, password_hash, role_id)
SELECT
    'Eleazar Fig',
    'eleazar.fig@example.com',
    '$2b$12$qgJr4V181Iitm5hndT64hO3BKfizSynM0MshNiq/lIL7AwP0ZU9xu',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';
