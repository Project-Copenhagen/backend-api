-- Add up migration script here
CREATE TABLE IF NOT EXISTS Users (
    Id text NOT NULL UNIQUE,
    Username text NOT NULL,
    BirthDate text NOT NULL,
    Gender text NOT NULL,
    Sexuality text NOT NULL,
    Notes text,
);