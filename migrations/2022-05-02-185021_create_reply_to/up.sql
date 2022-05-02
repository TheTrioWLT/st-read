CREATE TABLE ReplyTo (
    parent_comment INT,
    child_comment INT,
    PRIMARY KEY (parent_comment, child_comment),
    FOREIGN KEY (parent_comment) REFERENCES PostComment(comment_id),
    FOREIGN KEY (child_comment) REFERENCES PostComment(comment_id)
);
