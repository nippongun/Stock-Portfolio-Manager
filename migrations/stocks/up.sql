-- Your SQL goes here
CREATE TABLE "stocks" (
	"id"	INTEGER NOT NULL,
	"user_id"	INTEGER NOT NULL,
	"ticker"	TEXT NOT NULL,
	"purchase_price"	REAL NOT NULL,
	"current_price"	REAL NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);