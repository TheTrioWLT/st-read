CREATE TABLE PostReaction (
	userID INT NOT NULL,
	postID INT NOT NULL,
	upvote BOOLEAN NOT NULL,
	PRIMARY KEY (userID, postID)
);
