CREATE TABLE PostComments (
	userID INT NOT NULL,
	commentID INT NOT NULL,
	PRIMARY KEY (userID, commentID)
);
