use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use st_read::models::{NewPost, PostComment, PostCommentOn, PostComments, Posts, User};
use st_read::models::{Post as DbPost, ReplyTo};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use crate::{
    create_post::CreatePostFrame,
    posts_list::PostsListFrame,
    profile::UserProfileFrame,
    viewing_post::{Comment, ViewingPostFrame},
};

#[derive(Debug, Clone, Copy)]
pub enum AppView {
    Homepage,
    UserProfile,
    CreatePost,
}

#[derive(Debug, Clone, Copy)]
pub enum SelectedFrame {
    Posts,
    ViewPost,
}

#[derive(Debug, Clone)]
pub struct Post {
    pub title: String,
    pub short: String,
    pub stats: String,
    pub author: String,
    pub full: String,
    pub comments: Vec<Comment>,
}

/// This struct holds the current state of the app.
pub struct App {
    pub page_title: PageTitle,
    view: AppView,
    pub posts_frame: PostsListFrame,
    pub selected_frame: SelectedFrame,
    pub viewing_frame: ViewingPostFrame,
    pub profile_frame: UserProfileFrame,
    pub create_frame: CreatePostFrame,
    pub quittable: bool,
}

impl App {
    pub fn new(email: String, password: String) -> Self {
        let user = Self::authenticate(&email, password).unwrap();
        let posts = Self::load_posts();
        App {
            page_title: PageTitle::new("Homepage"),
            posts_frame: PostsListFrame::with_items(posts),
            viewing_frame: ViewingPostFrame::new(),
            view: AppView::Homepage,
            create_frame: CreatePostFrame::new(),
            selected_frame: SelectedFrame::Posts,
            quittable: true,
            profile_frame: UserProfileFrame {
                selected: crate::profile::SelectedOption::None,
                dark_mode: user.dark_mode,
                user_id: user.user_id,
                email_notifications: user.email_notifications,
                email,
                name: user.name,
            },
        }
    }

    pub fn submit_post(&self, title: &str, text: &str) {
        let connection = st_read::establish_connection();
        use st_read::schema::post::dsl as post_dsl;
        use st_read::schema::posts::dsl as posts_dsl;

        let post = NewPost {
            title: title.to_owned(),
            text: text.to_owned(),
        };

        let post: DbPost = post
            .insert_into(post_dsl::post)
            .get_result(&connection)
            .unwrap();

        let author_id = self.profile_frame.user_id;
        let posts = Posts {
            user_id: author_id,
            post_id: post.post_id,
        };
        posts
            .insert_into(posts_dsl::posts)
            .execute(&connection)
            .unwrap();
    }

    pub fn reload_posts(&mut self) {
        let posts = Self::load_posts();
        self.posts_frame = PostsListFrame::with_items(posts);
    }

    fn load_posts() -> Vec<Post> {
        use st_read::schema::post::dsl as post_dsl;
        use st_read::schema::postcomment::dsl as post_comment_dsl;
        use st_read::schema::postcomment::dsl::comment_id as post_comment_comment_id;
        use st_read::schema::postcommenton::dsl as post_comment_on_dsl;
        use st_read::schema::postcommenton::post_id as post_comment_on_id;
        use st_read::schema::postcomments::dsl as post_comments_dsl;
        use st_read::schema::postcomments::dsl::comment_id as post_comments_comment_id;
        use st_read::schema::posts::dsl as posts_dsl;
        use st_read::schema::posts::dsl::post_id as post_id_dsl;
        use st_read::schema::users::dsl as users_dsl;
        use st_read::schema::users::dsl::user_id as user_id_dsl;

        let connection = st_read::establish_connection();
        let posts = post_dsl::post.get_results::<DbPost>(&connection).unwrap();

        posts
            .into_iter()
            .map(|p| {
                let mut short = p.text[..16.min(p.text.len())].to_owned();
                short.push_str(" ...");
                let stats = "STATS TODO".to_owned();
                let author_id = posts_dsl::posts
                    .filter(post_id_dsl.eq(p.post_id))
                    .first::<Posts>(&connection)
                    .unwrap_or_else(|e| {
                        panic!("Failed to load author with post id {}: {}", p.post_id, e)
                    });

                let author: User = users_dsl::users
                    .filter(user_id_dsl.eq(author_id.user_id))
                    .first(&connection)
                    .unwrap_or_else(|e| {
                        panic!("Failed to find user id {}: {}", author_id.user_id, e)
                    });

                let author = author.name.clone();

                let root_comments: Vec<PostCommentOn> = post_comment_on_dsl::postcommenton
                    .filter(post_comment_on_id.eq(p.post_id))
                    .get_results(&connection)
                    .unwrap();

                fn transform_comment(id: i32) -> Comment {
                    let connection = st_read::establish_connection();
                    let post_comment: PostComment = post_comment_dsl::postcomment
                        .filter(post_comment_comment_id.eq(id))
                        .first(&connection)
                        .unwrap();
                    let comment: PostComments = post_comments_dsl::postcomments
                        .filter(post_comments_comment_id.eq(id))
                        .first(&connection)
                        .unwrap();

                    let comment_author: User = users_dsl::users
                        .filter(user_id_dsl.eq(comment.user_id))
                        .first(&connection)
                        .unwrap_or_else(|e| panic!("Failed to find user id {}: {}", id, e));

                    Comment::new(post_comment.text, &comment_author.name, vec![])
                }

                let comments: Vec<(_, _)> = root_comments
                    .into_iter()
                    .map(|base_comment| {
                        (
                            transform_comment(base_comment.comment_id),
                            base_comment.comment_id,
                        )
                    })
                    .collect();

                fn load_recursive_comments(id: i32) -> Vec<Comment> {
                    let connection = st_read::establish_connection();
                    use st_read::schema::replyto::dsl as reply_to_dsl;
                    use st_read::schema::replyto::parent_comment as reply_to_parent_comment_dsl;

                    let replies: Vec<ReplyTo> = reply_to_dsl::replyto
                        .filter(reply_to_parent_comment_dsl.eq(id))
                        .get_results(&connection)
                        .unwrap();

                    replies
                        .into_iter()
                        .map(|c| {
                            let mut comment = transform_comment(c.child_comment);
                            comment.children = load_recursive_comments(c.child_comment);
                            comment
                        })
                        .collect()
                }

                let comments = comments
                    .into_iter()
                    .map(|(mut comment, id)| {
                        let children = load_recursive_comments(id);
                        comment.children = children;

                        comment
                    })
                    .collect();

                Post {
                    title: p.title,
                    short,
                    stats,
                    author,
                    full: p.text,
                    comments,
                }
            })
            .collect()
    }

    fn authenticate(email: &str, mut password: String) -> Result<User, ()> {
        use st_read::schema::users::dsl as users_dsl;
        use st_read::schema::users::dsl::email as email_dsl;
        let connection = st_read::establish_connection();

        let user: User = users_dsl::users
            .filter(email_dsl.eq(&email))
            .first(&connection)
            .map_err(|e| panic!("{}", e))?;

        let hash = st_read::util::hash(&password, &user.name);
        let correct = user.password_hash.as_slice() == &hash;

        //Don't keep passwords in memory even when string is dropped
        zeroize::Zeroize::zeroize(&mut password);
        match correct {
            true => Ok(user),
            false => Err(()),
        }
    }

    pub fn set_view(&mut self, view: AppView) {
        self.view = view;

        match view {
            AppView::Homepage => {
                self.page_title.set_title("Homepage");
            }
            AppView::UserProfile => {
                self.page_title.set_title("User Profile");
            }
            AppView::CreatePost => {
                self.page_title.set_title("Creating Post");
            }
        }
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<App> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if app.quittable && matches!(key.code, KeyCode::Char('q')) {
                    return Ok(app);
                }

                match app.view {
                    AppView::Homepage => match app.selected_frame {
                        SelectedFrame::Posts => {
                            PostsListFrame::handle_key(&mut app, key);
                        }
                        SelectedFrame::ViewPost => {
                            ViewingPostFrame::handle_key(&mut app, key);
                        }
                    },
                    AppView::UserProfile => {
                        UserProfileFrame::handle_key(&mut app, key);
                    }
                    AppView::CreatePost => {
                        CreatePostFrame::handle_key(&mut app, key);
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(7), Constraint::Percentage(93)].as_ref())
        .split(f.size());

    app.page_title.render(f, vertical[0]);

    if matches!(app.view, AppView::Homepage) {
        // Create two chunks with equal horizontal screen space
        let constraints = if app.viewing_frame.has_post() {
            [Constraint::Percentage(50), Constraint::Percentage(50)].as_ref()
        } else {
            [Constraint::Percentage(100)].as_ref()
        };

        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(vertical[1]);

        app.posts_frame.render(
            f,
            horizontal[0],
            matches!(app.selected_frame, SelectedFrame::Posts),
        );

        if app.viewing_frame.has_post() {
            app.viewing_frame.render(
                f,
                horizontal[1],
                matches!(app.selected_frame, SelectedFrame::ViewPost),
            );
        }
    } else if matches!(app.view, AppView::UserProfile) {
        app.profile_frame.render(f, vertical[1]);
    } else if matches!(app.view, AppView::CreatePost) {
        app.create_frame.render(f, vertical[1]);
    }
}

pub fn get_border_style(selected: bool, locked: bool) -> Style {
    let style = Style::default();

    if selected {
        if locked {
            style.fg(Color::LightMagenta)
        } else {
            style.fg(Color::LightGreen)
        }
        .add_modifier(Modifier::BOLD)
    } else {
        style.fg(Color::Gray)
    }
}

pub struct PageTitle {
    pub title: String,
}

impl PageTitle {
    pub fn new(title: impl AsRef<str>) -> Self {
        Self {
            title: String::from(title.as_ref()),
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let page_title_block = Paragraph::new(self.title.clone())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::LightCyan)),
            )
            .style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .wrap(tui::widgets::Wrap { trim: false });

        f.render_widget(page_title_block, area);
    }

    pub fn set_title(&mut self, title: impl AsRef<str>) {
        self.title = String::from(title.as_ref());
    }
}
