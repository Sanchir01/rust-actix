-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS colors (
    id UUID DEFAULT uuid_generate_v4 () PRIMARY KEY,
    title TEXT NOT NULL,
    slug TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version BIGINT NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS candles (
    id UUID DEFAULT uuid_generate_v4 () PRIMARY KEY,
    title TEXT NOT NULL,
    slug TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version BIGINT NOT NULL DEFAULT 1,
    price INT NOT NULL,
    color_id UUID NOT NULL,
    FOREIGN KEY (color_id) REFERENCES colors (id) ON DELETE CASCADE
);

-- +goose StatementEnd
-- +goose Down
-- +goose StatementBegin
SELECT
    'down SQL query';

-- +goose StatementEnd
