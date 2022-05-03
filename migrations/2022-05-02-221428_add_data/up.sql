
INSERT INTO Users
    (email, name, password_hash)
    VALUES
    ('troyneubauer@gmail.com', 'Troy Neubauer', decode('TvLBjkSzTiiZG84zypeRcIlDUqJUF9ZFn6LHyZUnaDBYyLUYzusAH3JE0jPxSH2sBUU561esM6WAVxIDtJXqrQ==', 'base64'));

INSERT INTO Users
    (email, name, password_hash)
    VALUES
    ('newcomb.luke@protonmail.com', 'Luke Newcomb', decode('9d9ZyEDBVoVFXiT+JUyhUChwcQiKKCTKPTXpvyeRpDciNReYAfdjbUSAmyRRssDCUWkihY4lTIbqwL1dQF7ajg==', 'base64'));

INSERT INTO Users
    (email, name, password_hash)
    VALUES
    ('illusion173@hotmail.com', 'Jeremiah Webb', decode('WY9o9JG77pEELi7LZbTTWrONuwBpQKnwH/KQxz1WFM11sGGvFvhWhEeXPzRu7dxffeFXJsot4HmPOHh6/AR7Vw==', 'base64'));

INSERT INTO Users
    (email, name, password_hash)
    VALUES
    ('jalesan@my.erau.edu', 'Joe Alesandrini', 'TODO');

INSERT INTO Users
    (email, name, password_hash)
    VALUES
    ('plopez@my.erau.edu', 'Pablo Lopez', 'TODO');

INSERT INTO Users
    (email, name, password_hash)
    VALUES
    ('nrodrig@my.erau.edu', 'Nick Rodriguez', 'TODO');

INSERT INTO Users
    (email, name, password_hash)
    VALUES
    ('msanders@my.erau.edu', 'Gage Sanderson', 'TODO');

INSERT INTO Users
    (email, name, password_hash)
    VALUES
    ('rhoke5@my.erau.edu', 'RoseEllen Hoke', 'TODO');

INSERT INTO Users
    (email, name, password_hash)
    VALUES
    ('whernan6@my.erau.edu', 'Walter Hernandez', decode('dNlEFYOQF055+/xKjfV7e9hVqXCvGP/xRcEvv7fNDKAji0GcMi84zW+3w+L6maIQ97hJvkc93lablFzD9YMS6w==', 'base64'));


-- ##############################  COMMENTS  ##############################
INSERT INTO PostComment
    (comment_id, text)
    VALUES
    (1, 'As deserved for a linux user lmao. "I use arch btw!". Get out of here');

INSERT INTO PostComment
    (comment_id, text)
    VALUES
    (2, 'Glad to be here! Long live ST-READ!');

INSERT INTO PostComment
    (comment_id, text)
    VALUES
    (3, 'My grandpa recorded all of the original Wizard of Oz books on audiocassettes, before "books on tape" were really a thing, and gave them to me along with the books while I was learning to read, so I could read along. I read/listened to all of them, but I didn"t understand how special they were at the time, and after I listened to them I recorded over them. :"( He had a wonderful voice and he"s been gone for almost 10 years now... especially now that I have a daughter of my own I"d pay a million bucks to get those recordings back. I have videos of him and such, but those tapes were such a loving gesture and they took him hours to make. I"m still hoping I"ll find one of them with at least part of a recording in my parents" attic or something.');

INSERT INTO PostComment
    (comment_id, text)
    VALUES
    (4, 'Ouch.');

INSERT INTO PostComment
    (comment_id, text)
    VALUES
    (5, 'For real, man that pained me to read. May sound dramatic to some, but that really is devastating.');

INSERT INTO PostComment
    (comment_id, text)
    VALUES
    (6, '“And if you click your heels 3 times and make a wish, you’ll find your way homeYO WADDUP HOMIES DROPPING MY LATEST MIXTAPE”');

INSERT INTO PostComment
    (comment_id, text)
    VALUES
    (8, 'I think I"ll just use the sink.');

INSERT INTO PostComment
    (comment_id, text)
    VALUES
    (7, 'There was a time when my family named our toilet "John" and whenever my sister was taking too long in there we"d make up stories about how she was cheating on her boyfriend with John.This brings back some memories.');

INSERT INTO PostComment
    (comment_id, text)
    VALUES
    (9, 'Isn"t it better to name your toilet "Jim" instead? Because then you can claim that you go to the Jim everyday.');

INSERT INTO PostComment
    (comment_id, text)
    VALUES
    (10, 'Ok that’s enough Reddit for today');


-- ##############################  POST  ##############################

INSERT INTO Post
    (post_id, date_posted, title, text)
    VALUES
    (1, CURRENT_TIMESTAMP, 'Hello TS-Read!!!', 'Hello ts-readWeclome to our new site! We have built this as a open marketplace of ideas, where users are free to discuss issues and share information in a secure way.');

INSERT INTO Post
    (post_id, date_posted, title, text)
    VALUES
    (2, CURRENT_TIMESTAMP, 'I hate Windows', 'I have had enough with windows. Today my machine auto-updated to Windows 11, deleting my linux partitian in the process. Microsoft is a terible instutition that bringn about tyranny on the world with their bad products andn worse buisness pratices');

INSERT INTO Post
    (post_id, date_posted, title, text)
    VALUES
    (3, CURRENT_TIMESTAMP, 'Your toilet is now sentient. Would you prefer it to passionately, exuberantly crave your excrement, or deeply resent and despise you for what you do to it? Why?', '');

INSERT INTO Post
    (post_id, date_posted, title, text)
    VALUES
    (4, CURRENT_TIMESTAMP, 'What"s one thing you had as a child that you wish you had now?', '');



-- ##############################  COMMENT ON  ##############################

INSERT INTO PostCommentOn
    (comment_id, post_id)
    VALUES
    (1, 2);

INSERT INTO PostCommentOn
    (comment_id, post_id)
    VALUES
    (2, 1);

INSERT INTO PostCommentOn
    (comment_id, post_id)
    VALUES
    (3, 4);

INSERT INTO PostCommentOn
    (comment_id, post_id)
    VALUES
    (6, 4);

INSERT INTO PostCommentOn
    (comment_id, post_id)
    VALUES
    (7, 3);

INSERT INTO PostCommentOn
    (comment_id, post_id)
    VALUES
    (8, 3);

INSERT INTO PostCommentOn
    (comment_id, post_id)
    VALUES
    (10, 3);

-- ##############################  COMMENT REACTION  ##############################

INSERT INTO CommentReaction
    (user_id, comment_id, upvote)
    VALUES
    (9, 1, false);

INSERT INTO CommentReaction
    (user_id, comment_id, upvote)
    VALUES
    (3, 1, true);

INSERT INTO CommentReaction
    (user_id, comment_id, upvote)
    VALUES
    (5, 4, false);

INSERT INTO CommentReaction
    (user_id, comment_id, upvote)
    VALUES
    (6, 5, true);



-- ##############################  COMMENTS  ##############################

--Jeremiah linux comment
INSERT INTO PostComments
    (user_id, comment_id)
    VALUES
    (3, 1);

--Walter welcome comment
INSERT INTO PostComments
    (user_id, comment_id)
    VALUES
    (9, 2);

--Grandpa recordings when Joe was little
INSERT INTO PostComments
    (user_id, comment_id)
    VALUES
    (4, 3);

--Ouch
INSERT INTO PostComments
    (user_id, comment_id)
    VALUES
    (6, 4);

--That hurt to read
INSERT INTO PostComments
    (user_id, comment_id)
    VALUES
    (5, 5);

--wizard of oz joke
INSERT INTO PostComments
    (user_id, comment_id)
    VALUES
    (7, 6);

--toilet sink joke
INSERT INTO PostComments
    (user_id, comment_id)
    VALUES
    (8, 7);

--John toilet family story
INSERT INTO PostComments
    (user_id, comment_id)
    VALUES
    (4, 8);

--Better to name toilet Jim
INSERT INTO PostComments
    (user_id, comment_id)
    VALUES
    (3, 9);

--Thats enough reddit for today
INSERT INTO PostComments
    (user_id, comment_id)
    VALUES
    (9, 10);



-- ##############################  POSTS  ##############################

INSERT INTO posts (user_id, post_id) VALUES (1, 1);
INSERT INTO posts (user_id, post_id) VALUES (2, 2);
INSERT INTO posts (user_id, post_id) VALUES (3, 3);
INSERT INTO posts (user_id, post_id) VALUES (4, 4);


-- ##############################  POST REACTIONS  ##############################

INSERT INTO PostReaction
    (user_id, post_id, upvote)
    VALUES
    (1, 1, true);

INSERT INTO PostReaction
    (user_id, post_id, upvote)
    VALUES
    (2, 2, true);

INSERT INTO PostReaction
    (user_id, post_id, upvote)
    VALUES
    (3, 3, false);

INSERT INTO PostReaction
    (user_id, post_id, upvote)
    VALUES
    (3, 4, true);

-- ##############################  REPLY TO  ##############################

INSERT INTO ReplyTo
    (parent_comment, child_comment)
    VALUES
    (3, 4);

INSERT INTO ReplyTo
    (parent_comment, child_comment)
    VALUES
    (4, 5);

INSERT INTO ReplyTo
    (parent_comment, child_comment)
    VALUES
    (8, 9);

