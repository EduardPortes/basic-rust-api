CREATE TABLE user (
  id SERIAL PRIMARY KEY,
  username VARCHAR(50) NOT NULL UNIQUE,
  email VARCHAR(100) NOT NULL UNIQUE,
  password VARCHAR(255) NOT NULL
);

INSERT INTO user (username, email, password) VALUES
('admin', 'admin@gmail.com', '$2b$10$EIX/5z1Z3f8a1e5d9Q0uOe5h6j7k5l8m9n0o1p2q3r4s5t6u7v8w9y'),
('user1', 'user1@gmail.com', '$2b$10$EIX/5z1Z3f8a1e5d9Q0uOe5h6j7k5l8m9n0o1p2q3r4s5t6u7v8w9y');