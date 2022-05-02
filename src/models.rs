use std::time::SystemTime;

#[derive(Queryable)]
pub struct User {
    pub user_id: i32,
    pub email: String,
    pub name: String,
    pub password_hash: Vec<u8>,
    pub dark_mode: bool,
    pub email_notifications: bool,
}

use super::schema::post;

#[derive(Queryable, Insertable, Debug)]
#[table_name="post"]
pub struct Post {
    pub post_id: i32,
    pub date_posted: SystemTime,
    pub title: String,
    pub text: String,
}

#[derive(Queryable)]
pub struct Posts {
    pub user_id: i32,
    pub post_id: i32,
}

#[derive(Queryable)]
pub struct PostComment {
    pub comment_id: i32,
    pub text: String,
}

#[derive(Queryable)]
pub struct PostComments {
    pub user_id: i32,
    pub comment_id: i32,
}

#[derive(Queryable)]
pub struct PostCommentOn {
    pub comment_id: i32,
    pub post_id: i32,
}

#[derive(Queryable)]
pub struct ReplyTo {
    pub parent_comment: i32,
    pub child_comment: i32,
}

#[derive(Queryable)]
pub struct PostReaction {
    pub user_id: i32,
    pub post_id: i32,
    pub upvote: bool,
}

#[derive(Queryable)]
pub struct CommentReaction {
    pub user_id: i32,
    pub comment_id: i32,
    pub upvote: bool,
}
