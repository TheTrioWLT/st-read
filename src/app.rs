use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use crate::{
    create_post::CreatePostFrame,
    initial::InitialFrame,
    login::LoginFrame,
    posts_list::PostsListFrame,
    profile::UserProfileFrame,
    register::RegisterFrame,
    viewing_post::{Comment, ViewingPostFrame},
};

#[derive(Debug, Clone, Copy)]
pub enum AppView {
    Homepage,
    UserProfile,
    CreatePost,
    Login,
    Register,
    Initial,
}

#[derive(Debug, Clone, Copy)]
pub enum SelectedFrame {
    Posts,
    ViewPost,
}

#[derive(Debug, Clone)]
pub struct Post {
    pub title: &'static str,
    pub short: &'static str,
    pub stats: &'static str,
    pub author: &'static str,
    pub full: &'static str,
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
    pub initial_frame: InitialFrame,
    pub login_frame: LoginFrame,
    pub register_frame: RegisterFrame,
    pub quittable: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            page_title: PageTitle::new("Welcome to ST-Read"),
            posts_frame: PostsListFrame::with_items(vec![
                    Post {
                        title: "My New Cat",
                        short: "I just purchased a new cat named Fluffy ...",
                        stats: "33 Comments, 152 Upvotes",
                        author: "Troy Neubauer",
                        full: "",
                        comments: Vec::new()
                    },
                    Post {
                        title: "Why I hate Windows",
                        short: "I've had enough with Windows ...",
                        stats: "91 Comments, 0 Upvotes",
                        author: "Luke Newcomb",
                        full: "I have had enough with Windows. Today my machine auto-updated to Windows 11! Deleting my entire Linux partition in the process.",
                        comments: vec![Comment::new("As deserved for a Linux user lmao. \"I use Arch BTW.\" Get out of here.", "Jeremiah Webb")]
                    },
                    Post {
                        title: "Hello ST-Read",
                        short: "Hello ST-read. Welcome to our new site! ...",
                        stats: "12 Comments, 4 Upvotes",
                        author: "Troy Neubauer",
                        full: "",
                        comments: Vec::new()
                    },
            ]),
            viewing_frame: ViewingPostFrame::new(),
            view: AppView::Initial,
            create_frame: CreatePostFrame::new(),
            selected_frame: SelectedFrame::Posts,
            profile_frame: UserProfileFrame::new(),
            initial_frame: InitialFrame::new(),
            login_frame: LoginFrame::new(),
            register_frame: RegisterFrame::new(),
            quittable: true
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
            AppView::Initial => {
                self.page_title.set_title("Welcome to ST-Read");
            }
            AppView::Login => {
                self.page_title.set_title("Login");
            }
            AppView::Register => {
                self.page_title.set_title("Register");
            }
        }
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
                    AppView::Initial => {
                        InitialFrame::handle_key(&mut app, key);
                    }
                    AppView::Login => {
                        LoginFrame::handle_key(&mut app, key);
                    }
                    AppView::Register => {
                        RegisterFrame::handle_key(&mut app, key);
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
    } else if matches!(app.view, AppView::Initial) {
        app.initial_frame.render(f, vertical[1]);
    } else if matches!(app.view, AppView::Register) {
        app.register_frame.render(f, vertical[1]);
    } else if matches!(app.view, AppView::Login) {
        app.login_frame.render(f, vertical[1]);
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
