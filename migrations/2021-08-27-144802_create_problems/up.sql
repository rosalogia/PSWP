-- Your SQL goes here
CREATE TABLE user_accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL 
);

CREATE TABLE problems (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    author_id INTEGER NOT NULL,
    name TEXT UNIQUE NOT NULL,
    description TEXT NOT NULL,
    test_case_file TEXT NOT NULL,
    FOREIGN KEY(author_id) REFERENCES user_account(id)
);
