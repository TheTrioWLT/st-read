CREATE TABLE post_reaction (
    userID INT,
    postID INT,
    upvote BOOLEAN,
    PRIMARY KEY (userID, postID),
    FOREIGN KEY (userID) REFERENCES users(userID),
    FOREIGN KEY (postID) REFERENCES post(postID)
);
