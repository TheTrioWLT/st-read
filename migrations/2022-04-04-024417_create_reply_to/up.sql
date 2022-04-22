CREATE TABLE ReplyTo (
	parentComment INT NOT NULL,
	childComment INT NOT NULL,
	PRIMARY KEY (parentComment, childComment)
);
