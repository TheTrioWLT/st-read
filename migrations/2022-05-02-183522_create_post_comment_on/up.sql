CREATE TABLE PostCommentOn (
    comment_id INTEGER,
    post_id INTEGER,
    PRIMARY KEY (comment_id, post_id),
    FOREIGN KEY (comment_id) REFERENCES PostComment(comment_id),
    FOREIGN KEY (post_id) REFERENCES Post(post_id)
);
