CREATE TABLE post_comment_on (
    commentID INT,
    postID INT,
    PRIMARY KEY (commentID, postID),
    FOREIGN KEY (commentID) REFERENCES post_comment(commentID),
    FOREIGN KEY (postID) REFERENCES post(postID)
);
