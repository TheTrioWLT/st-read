table! {
    commentreaction (user_id, comment_id) {
        user_id -> Int4,
        comment_id -> Int4,
        upvote -> Nullable<Bool>,
    }
}

table! {
    post (post_id) {
        post_id -> Int4,
        date_posted -> Timestamp,
        title -> Text,
        text -> Text,
    }
}

table! {
    postcomment (comment_id) {
        comment_id -> Int4,
        text -> Text,
    }
}

table! {
    postcommenton (comment_id, post_id) {
        comment_id -> Int4,
        post_id -> Int4,
    }
}

table! {
    postcomments (user_id, comment_id) {
        user_id -> Int4,
        comment_id -> Int4,
    }
}

table! {
    postreaction (user_id, post_id) {
        user_id -> Int4,
        post_id -> Int4,
        upvote -> Nullable<Bool>,
    }
}

table! {
    posts (user_id, post_id) {
        user_id -> Int4,
        post_id -> Int4,
    }
}

table! {
    replyto (parent_comment, child_comment) {
        parent_comment -> Int4,
        child_comment -> Int4,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        email -> Varchar,
        name -> Varchar,
        password_hash -> Bytea,
        dark_mode -> Bool,
        email_notifications_enabled -> Bool,
    }
}

joinable!(commentreaction -> postcomment (comment_id));
joinable!(commentreaction -> users (user_id));
joinable!(postcommenton -> post (post_id));
joinable!(postcommenton -> postcomment (comment_id));
joinable!(postcomments -> postcomment (comment_id));
joinable!(postcomments -> users (user_id));
joinable!(postreaction -> post (post_id));
joinable!(postreaction -> users (user_id));
joinable!(posts -> post (post_id));
joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    commentreaction,
    post,
    postcomment,
    postcommenton,
    postcomments,
    postreaction,
    posts,
    replyto,
    users,
);
