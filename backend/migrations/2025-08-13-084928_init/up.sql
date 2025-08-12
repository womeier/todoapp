-- Your SQL goes here
CREATE TABLE `tasks`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`title` TEXT NOT NULL,
	`done` BOOL NOT NULL,
	`label` TEXT NOT NULL
);

