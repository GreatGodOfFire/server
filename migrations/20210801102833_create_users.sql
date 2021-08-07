CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    enc_cert bytea UNIQUE,
    username text UNIQUE NOT NULL,
    master_pw_hash text NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT current_timestamp,
    suspended bool NOT NULL DEFAULT false,
    suspended_reason TEXT DEFAULT NULL
)