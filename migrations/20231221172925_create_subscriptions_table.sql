-- Add migration script here
-- Create Subscriptions table
CREATE TABLE Subscriptions(
       id uuid NOT NULL,
       PRIMARY KEY (id),
       email TEXT NOT NULL UNIQUE,
       name TEXT NOT NULL,
       subscribed_at timestamptz NOT NULL
);
