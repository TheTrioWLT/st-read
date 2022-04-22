table! {
    CommentReaction (userID, commentID) {
        userID -> Integer,
        commentID -> Integer,
        upvote -> Bool,
    }
}

table! {
    Post (postID) {
        postID -> Integer,
        datePosted -> Datetime,
        title -> Varchar,
        text -> Varchar,
    }
}

table! {
    PostComment (commentID) {
        commentID -> Integer,
        text -> Varchar,
    }
}

table! {
    PostCommentOn (commentID, postID) {
        commentID -> Integer,
        postID -> Integer,
    }
}

table! {
    PostComments (userID, commentID) {
        userID -> Integer,
        commentID -> Integer,
    }
}

table! {
    PostReaction (userID, postID) {
        userID -> Integer,
        postID -> Integer,
        upvote -> Bool,
    }
}

table! {
    Posts (userID, postID) {
        userID -> Integer,
        postID -> Integer,
    }
}

table! {
    ReplyTo (parentComment, childComment) {
        parentComment -> Integer,
        childComment -> Integer,
    }
}

table! {
    User (userID) {
        userID -> Integer,
        email -> Varchar,
        name -> Varchar,
        passwordHash -> Varbinary,
        darkMode -> Bool,
        emailNotificationsEnabled -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    CommentReaction,
    Post,
    PostComment,
    PostCommentOn,
    PostComments,
    PostReaction,
    Posts,
    ReplyTo,
    User,
);
