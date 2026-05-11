-- Seed data for local development
-- superadmin@test.com / password123
INSERT INTO users (id, email, name, password_hash)
VALUES (
    'a0000000-0000-0000-0000-000000000001',
    'superadmin@test.com',
    'Super Admin',
    '$argon2id$v=19$m=19456,t=2,p=1$XhNp4qqTkUArTvAs5h+Dxw$luif6WH1eoqQ7OvihJKQ6IQTU2McvQBysAMrBIPX5u0'
)
ON CONFLICT (email) DO NOTHING;
