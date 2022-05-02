CREATE TABLE post_comments (
    userID INT NOT NULL,
    commentID INT NOT NULL,
    PRIMARY KEY (userID, commentID),
    FOREIGN KEY (userID) REFERENCES users(userID),
    FOREIGN KEY (commentID) REFERENCES post_comment(commentID)
);
