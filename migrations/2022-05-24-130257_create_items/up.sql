-- Your SQL goes here
CREATE TABLE items ( -- table containing all item specific data
	id INT PRIMARY KEY NOT NULL,
	intervall INT NOT NULL, -- spaced-rs requires below data, Intervall in days
	difficulty FLOAT NOT NULL,
	memory_strength FLOAT NOT NULL,
	adjusting_factor FLOAT NOT NULL,
	times_reviewed INT NOT NULL,
	times_recalled INT NOT NULL, -- spaced-rs requires above data
	url TEXT NOT NULL UNIQUE, -- link to problem
	item_data TEXT NOT NULL -- text containing notes on problem
)
