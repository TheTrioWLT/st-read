CREATE TABLE posts (
    userID INT,
    postID INT,
    PRIMARY KEY (userID, postID),
    FOREIGN KEY (userID) REFERENCES users(userID),
    FOREIGN KEY (postID) REFERENCES post(postID)
);
