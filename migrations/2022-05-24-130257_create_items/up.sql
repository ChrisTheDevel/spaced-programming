-- Your SQL goes here
CREATE TABLE items (
	id INT PRIMARY KEY NOT NULL,
	intervall INT NOT NULL,
	difficulty FLOAT NOT NULL,
	memory_strength FLOAT NOT NULL,
	adjusting_factor FLOAT NOT NULL,
	times_reviewed INT NOT NULL,
	times_recalled INT NOT NULL,
	item_type TEXT NOT NULL,
	item_data TEXT NOT NULL
)
