CREATE TABLE post (
    postID SERIAL PRIMARY KEY,
    datePosted TIMESTAMP NOT NULL,
    title VARCHAR(40) NOT NULL,
    text VARCHAR(1024) NOT NULL
);
