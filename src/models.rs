use std::time::SystemTime;

#[derive(Queryable)]
pub struct User {
    pub userID: i32,
    pub email: String,
    pub name: String,
    pub passwordHash: Vec<u8>,
    pub darkMode: bool,
    pub emailNotifications: bool,
}

use super::schema::post;

#[derive(Queryable, Insertable)]
#[table_name="post"]
pub struct Post {
    pub postid: i32,
    pub dateposted: SystemTime,
    pub title: String,
    pub text: String,
}

#[derive(Queryable)]
pub struct Posts {
    pub userID: i32,
    pub postID: i32,
}

#[derive(Queryable)]
pub struct PostComment {
    pub commentID: i32,
    pub text: String,
}

#[derive(Queryable)]
pub struct PostComments {
    pub userID: i32,
    pub commentID: i32,
}

#[derive(Queryable)]
pub struct PostCommentOn {
    pub commentID: i32,
    pub postID: i32,
}

#[derive(Queryable)]
pub struct ReplyTo {
    pub parentComment: i32,
    pub childComment: i32,
}

#[derive(Queryable)]
pub struct PostReaction {
    pub userID: i32,
    pub postID: i32,
    pub upvote: bool,
}

#[derive(Queryable)]
pub struct CommentReaction {
    pub userID: i32,
    pub commentID: i32,
    pub upvote: bool,
}
