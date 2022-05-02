CREATE TABLE ReplyTo (
    parent_comment INTEGER,
    child_comment INTEGER,
    PRIMARY KEY (parent_comment, child_comment),
    FOREIGN KEY (parent_comment) REFERENCES PostComment(comment_id),
    FOREIGN KEY (child_comment) REFERENCES PostComment(comment_id)
);
