CREATE TABLE ReplyTo (
	parentComment INT,
	childComment INT,
	PRIMARY KEY (parentComment, childComment)
);