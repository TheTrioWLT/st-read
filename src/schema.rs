table! {
    comment_reaction (userid, commentid) {
        userid -> Int4,
        commentid -> Int4,
        upvote -> Nullable<Bool>,
    }
}

table! {
    post (postid) {
        postid -> Int4,
        dateposted -> Timestamp,
        title -> Varchar,
        text -> Varchar,
    }
}

table! {
    post_comment (commentid) {
        commentid -> Int4,
        text -> Varchar,
    }
}

table! {
    post_comment_on (commentid, postid) {
        commentid -> Int4,
        postid -> Int4,
    }
}

table! {
    post_comments (userid, commentid) {
        userid -> Int4,
        commentid -> Int4,
    }
}

table! {
    post_reaction (userid, postid) {
        userid -> Int4,
        postid -> Int4,
        upvote -> Nullable<Bool>,
    }
}

table! {
    posts (userid, postid) {
        userid -> Int4,
        postid -> Int4,
    }
}

table! {
    reply_to (parentcomment, childcomment) {
        parentcomment -> Int4,
        childcomment -> Int4,
    }
}

table! {
    users (userid) {
        userid -> Int4,
    }
}

joinable!(comment_reaction -> post_comment (commentid));
joinable!(comment_reaction -> users (userid));
joinable!(post_comment_on -> post (postid));
joinable!(post_comment_on -> post_comment (commentid));
joinable!(post_comments -> post_comment (commentid));
joinable!(post_comments -> users (userid));
joinable!(post_reaction -> post (postid));
joinable!(post_reaction -> users (userid));
joinable!(posts -> post (postid));
joinable!(posts -> users (userid));

allow_tables_to_appear_in_same_query!(
    comment_reaction,
    post,
    post_comment,
    post_comment_on,
    post_comments,
    post_reaction,
    posts,
    reply_to,
    users,
);
