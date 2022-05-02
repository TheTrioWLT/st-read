CREATE TABLE PostReaction (
    user_id INT,
    post_id INT,
    upvote BOOLEAN,
    PRIMARY KEY (user_id, post_id),
    FOREIGN KEY (user_id) REFERENCES Users(user_id),
    FOREIGN KEY (post_id) REFERENCES Post(post_id)
);
