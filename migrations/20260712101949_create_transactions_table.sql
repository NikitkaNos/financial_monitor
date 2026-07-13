-- Add migration script here
-- Создаём таблицу пользователей
CREATE TABLE users(
    id uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);

-- Создаём таблицу транзакций
CREATE TABLE transactions(
    id uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount BIGINT NOT NULL, -- сумма в копейках
    category TEXT NOT NULL,
    type TEXT NOT NULL CHECK (type IN ('income', 'expense')),
    date DATE NOT NULL,
    description TEXT,
    created_at timestamptz NOT NULL DEFAULT now()
);

-- Индекс для быстрого поиска транзакций по пользователю и дате
CREATE INDEX transactions_user_id_date_idx ON transactions (user_id, date);