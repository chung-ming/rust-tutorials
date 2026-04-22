-- 1. Create a file named .env in your project root and add the following line:
-- DATABASE_URL="sqlite:sqlite.db"

-- 2. Use SQLx CLI to actually create the `sqlite.db` file:
-- Bash: sqlx database create

-- 3. Create a new migration for your posts table:
-- Bash: sqlx migrate add create_posts_table

-- 4. Add migration script here:
CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL
);

-- 5. Run the migration to apply this to your database:
-- Bash: sqlx migrate run

-- 6. Add sqlx crate to your project dependencies in cargo.toml:
-- sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "macros"] }
