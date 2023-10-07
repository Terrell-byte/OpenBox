CREATE TABLE chunks (
    chunk_id SERIAL PRIMARY KEY,
    chunk_data BYTEA NOT NULL,
    UNIQUE (chunk_data)
);

CREATE TABLE files (
    file_id SERIAL PRIMARY KEY,
    file_name VARCHAR(255) NOT NULL,
    UNIQUE (file_name)
);

CREATE TABLE chunk_references (
    reference_id SERIAL PRIMARY KEY,
    file_id INT REFERENCES files (file_id) ON DELETE CASCADE,
    chunk_id INT REFERENCES chunks (chunk_id),
    sequence_order INT NOT NULL
);

