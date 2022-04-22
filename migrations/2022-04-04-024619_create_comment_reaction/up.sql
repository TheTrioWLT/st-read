CREATE TABLE CommentReaction (
	userID INT NOT NULL,
	commentID INT NOT NULL,
	upvote BOOLEAN NOT NULL,
	PRIMARY KEY (userID, commentID)
);
