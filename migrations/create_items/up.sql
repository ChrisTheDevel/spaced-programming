-- Your SQL goes here
CREATE TABLE items ( -- table containing all item specific data
	id INTEGER PRIMARY KEY NOT NULL,
	intervall INTEGER NOT NULL, -- spaced-rs requires below data, Intervall in days
	difficulty REAL NOT NULL,
	memory_strength REAL NOT NULL,
	adjusting_factor REAL NOT NULL,
	times_reviewed INTEGER NOT NULL,
	times_recalled INTEGER NOT NULL, -- spaced-rs requires above data
	url TEXT NOT NULL UNIQUE, -- link to problem
	item_data TEXT NOT NULL -- text containing notes on problem
)
