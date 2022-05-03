CREATE TABLE PostComments (
    user_id INTEGER NOT NULL,
    comment_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, comment_id),
    FOREIGN KEY (user_id) REFERENCES Users(user_id),
    FOREIGN KEY (comment_id) REFERENCES PostComment(comment_id)
);
