CREATE TABLE CommentReaction (
	userID INT,
	commentID INT,
	upvote BOOLEAN,
	PRIMARY KEY (userID, commentID)
);