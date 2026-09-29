CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    author VARCHAR NOT NULL,
    isbn VARCHAR NOT NULL,
    book_rating INT NOT NULL CHECK (book_rating > 0 AND book_rating <= 5),
    genre VARCHAR DEFAULT 'Undefined',
    tags VARCHAR DEFAULT 'none',
    available BOOLEAN NOT NULL DEFAULT FALSE,
    count INT NOT NULL DEFAULT 0
);