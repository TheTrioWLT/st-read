CREATE TABLE comment_reaction (
    userID INT,
    commentID INT,
    upvote BOOLEAN,
    PRIMARY KEY (userID, commentID),
    FOREIGN KEY (userID) REFERENCES users(userID),
    FOREIGN KEY (commentID) REFERENCES post_comment(commentID)
);
