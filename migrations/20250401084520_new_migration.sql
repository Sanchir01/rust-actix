-- +goose Up
-- +goose StatementBegin
ALTER TABLE users ADD CONSTRAINT unique_email UNIQUE (email);
ALTER TABLE users ADD CONSTRAINT unique_phone UNIQUE (phone);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 'down SQL query';
-- +goose StatementEnd
