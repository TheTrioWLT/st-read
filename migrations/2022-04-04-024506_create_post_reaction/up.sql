CREATE TABLE PostReaction (
	userID INT,
	postID INT,
	upvote BOOLEAN,
	PRIMARY KEY (userID, postID)
);