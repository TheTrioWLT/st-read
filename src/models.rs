use std::time::SystemTime;

use super::schema::*;

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub user_id: i32,
    pub email: String,
    pub name: String,
    pub password_hash: Vec<u8>,
    pub dark_mode: bool,
    pub email_notifications_enabled: bool,
}

#[derive(Queryable, Debug)]
pub struct User {
    pub user_id: i32,
    pub email: String,
    pub name: String,
    pub password_hash: Vec<u8>,
    pub dark_mode: bool,
    pub email_notifications: bool,
}

#[derive(Queryable, Debug)]
pub struct Post {
    pub post_id: i32,
    pub date_posted: SystemTime,
    pub title: String,
    pub text: String,
}

#[derive(Insertable, Debug)]
#[table_name = "post"]
pub struct NewPost {
    pub title: String,
    pub text: String,
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "posts"]
pub struct Posts {
    pub user_id: i32,
    pub post_id: i32,
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "postcomment"]
pub struct PostComment {
    pub comment_id: i32,
    pub text: String,
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "postcomments"]
pub struct PostComments {
    pub user_id: i32,
    pub comment_id: i32,
}

#[derive(Queryable, Debug)]
pub struct PostCommentOn {
    pub comment_id: i32,
    pub post_id: i32,
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "replyto"]
pub struct ReplyTo {
    pub parent_comment: i32,
    pub child_comment: i32,
}

#[derive(Queryable, Debug)]
pub struct PostReaction {
    pub user_id: i32,
    pub post_id: i32,
    pub upvote: bool,
}

#[derive(Queryable, Debug)]
pub struct CommentReaction {
    pub user_id: i32,
    pub comment_id: i32,
    pub upvote: bool,
}
