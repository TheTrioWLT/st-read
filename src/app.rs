use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode};
use diesel::prelude::*;
use st_read::models::{Posts, User};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use crate::{
    posts_list::PostsListFrame,
    viewing_post::{Comment, ViewingPostFrame},
};

#[derive(Debug, Clone, Copy)]
pub enum AppView {
    Homepage,
    UserProfile,
}

#[derive(Debug, Clone, Copy)]
pub enum SelectedFrame {
    Posts,
    ViewPost,
    CreatePost,
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
    pub posts_frame: PostsListFrame,
    pub view: AppView,
    pub selected_frame: SelectedFrame,
    pub viewing_frame: ViewingPostFrame,
    pub quittable: bool,
}

impl App {
    pub fn new() -> Self {
        let posts = Self::load_posts();
        App {
            page_title: PageTitle::new("Homepage"),
            posts_frame: PostsListFrame::with_items(posts),
            viewing_frame: ViewingPostFrame::new(),
            view: AppView::Homepage,
            selected_frame: SelectedFrame::Posts,
            quittable: true,
        }
    }

    fn load_posts() -> Vec<Post> {
        use st_read::models::Post as DbPost;
        use st_read::schema::post::dsl as post_dsl;
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
                let author_id = match posts_dsl::posts
                    .filter(post_id_dsl.eq(p.post_id))
                    .first::<Posts>(&connection)
                {
                    Ok(user) => user.user_id,
                    Err(e) => panic!("Failed to load author with post id {}: {}", p.post_id, e),
                };

                let author: User = match users_dsl::users
                    .filter(user_id_dsl.eq(author_id))
                    .first(&connection)
                {
                    Ok(u) => u,
                    Err(e) => panic!("Failed to find user id {}: {}", author_id, e),
                };

                let author = author.name.clone();

                Post {
                    title: p.title,
                    short,
                    stats,
                    author,
                    full: p.text,
                    comments: vec![],
                }
            })
            .collect()
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if app.quittable && matches!(key.code, KeyCode::Char('q')) {
                    return Ok(());
                }

                match app.selected_frame {
                    SelectedFrame::Posts => {
                        PostsListFrame::handle_key(&mut app, key);
                    }
                    SelectedFrame::ViewPost => {
                        ViewingPostFrame::handle_key(&mut app, key);
                    }
                    SelectedFrame::CreatePost => unreachable!(),
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
    // Create two chunks with equal horizontal screen space
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(7), Constraint::Percentage(93)].as_ref())
        .split(f.size());

    let horizontal = if app.viewing_frame.has_post() {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(vertical[1])
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(vertical[1])
    };

    app.page_title.render(f, vertical[0]);

    if matches!(app.view, AppView::Homepage) {
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
    }
}

pub fn get_border_style(selected: bool, locked: bool) -> Style {
    if selected {
        if locked {
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD)
        }
    } else {
        Style::default().fg(Color::Gray)
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
