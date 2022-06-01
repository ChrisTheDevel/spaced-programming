-- This table contains items that are in a reviewing state.
CREATE TABLE schedule (
	id INTEGER PRIMARY KEY NOT NULL,
	due INTEGER NOT NULL, -- due date stored in unix time
	item_id INTEGER NOT NULL UNIQUE,
	FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE
)
