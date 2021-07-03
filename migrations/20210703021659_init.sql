SET TIME ZONE 'UTC';

CREATE TABLE IF NOT EXISTS tasks (
    id serial PRIMARY KEY UNIQUE NOT NULL,
    name varchar (63) NOT NULL,
    description varchar (255) NOT NULL,
    due_date bigint CHECK (due_date > 1609459200000),
    is_complete boolean DEFAULT 'false' NOT NULL
);
