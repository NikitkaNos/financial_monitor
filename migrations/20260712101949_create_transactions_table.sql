CREATE TABLE users(
    id uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);


-- Создаём счета
CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    account_type VARCHAR(20) NOT NULL CHECK (account_type IN ('Cash','Bank','Credit','Investment','External')),
    currency VARCHAR(3) NOT NULL DEFAULT 'RUB',
    opening_balance DECIMAL(15,2) NOT NULL DEFAULT 0.0,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Создаём категории с иерархией
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    type VARCHAR(10) NOT NULL CHECK (type IN ('Income','Expense')),
    parent_id UUID REFERENCES categories(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Создаём транзакции (шапка)
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Создаём проводки
CREATE TABLE transaction_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transaction_id UUID NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    signed_amount DECIMAL(15,2) NOT NULL
);

-- Индексы для производительности
CREATE INDEX accounts_user_id_idx ON accounts(user_id);
CREATE INDEX categories_user_id_parent_id_idx ON categories(user_id, parent_id);
CREATE INDEX transactions_user_id_date_idx ON transactions(user_id, date);
CREATE INDEX transaction_entries_transaction_id_idx ON transaction_entries(transaction_id);
CREATE INDEX transaction_entries_account_id_idx ON transaction_entries(account_id);

-- Триггер для автоматического обновления updated_at (можно позже добавить через функцию)
-- Пока пропустим, но заложим в коде ручное обновление.