CREATE TABLE reply_to (
    parentComment INT,
    childComment INT,
    PRIMARY KEY (parentComment, childComment),
    FOREIGN KEY (parentComment) REFERENCES post_comment(commentID),
    FOREIGN KEY (childComment) REFERENCES post_comment(commentID)
);
