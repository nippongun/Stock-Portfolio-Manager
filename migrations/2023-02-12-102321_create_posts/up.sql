-- Your SQL goes here
CREATE TABLE "posts" (
	"id"	INTEGER NOT NULL,
	"title"	BLOB NOT NULL,
	"body"	TEXT NOT NULL,
	"published"	INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);