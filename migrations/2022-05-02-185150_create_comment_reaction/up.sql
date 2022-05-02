CREATE TABLE CommentReaction (
    user_id INTEGER,
    comment_id INTEGER,
    upvote BOOLEAN,
    PRIMARY KEY (user_id, comment_id),
    FOREIGN KEY (user_id) REFERENCES Users(user_id),
    FOREIGN KEY (comment_id) REFERENCES PostComment(comment_id)
);
