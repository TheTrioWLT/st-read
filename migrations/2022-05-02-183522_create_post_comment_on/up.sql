CREATE TABLE PostCommentOn (
    comment_id INT,
    post_id INT,
    PRIMARY KEY (comment_id, post_id),
    FOREIGN KEY (comment_id) REFERENCES PostComment(comment_id),
    FOREIGN KEY (post_id) REFERENCES Post(post_id)
);
