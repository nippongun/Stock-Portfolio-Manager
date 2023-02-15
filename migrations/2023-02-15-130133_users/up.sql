-- Your SQL goes here
CREATE TABLE "users" (
	"id"	INTEGER NOT NULL,
	"user_name"	TEXT NOT NULL UNIQUE,
	"api_key"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);