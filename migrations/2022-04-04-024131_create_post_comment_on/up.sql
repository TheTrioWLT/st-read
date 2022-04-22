CREATE TABLE PostCommentOn (
	commentID INT NOT NULL,
	postID INT NOT NULL,
	PRIMARY KEY (commentID, postID)
);
