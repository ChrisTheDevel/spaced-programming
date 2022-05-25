-- This table contains items that are in a reviewing state.
CREATE TABLE schedule (
	id INT PRIMARY KEY NOT NULL,
	due INT NOT NULL, -- due date stored in unix time
	item_id INT NOT NULL,
	FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE
)
