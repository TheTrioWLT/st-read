use crossterm::event::{KeyCode, KeyEvent};
use diesel::prelude::*;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Borders, Paragraph, Wrap};
use tui::Frame;
use tui::{backend::Backend, widgets::Block};

use crate::app::{get_border_style, App, AppView};
use crate::initial::{InitialButtonBox, LoginSelectedBox};

pub struct RegisterFrame {
    username: UsernameBox,
    password: PasswordBox,
    name: NameBox,
    register_button: RegisterButtonBox,
    back_button: InitialButtonBox,
    selected: LoginSelectedBox,
}

impl RegisterFrame {
    pub fn new() -> Self {
        Self {
            username: UsernameBox::new(),
            password: PasswordBox::new(),
            name: NameBox::new(),
            register_button: RegisterButtonBox::new(),
            back_button: InitialButtonBox::new("Back", false),
            selected: LoginSelectedBox::Username,
        }
    }

    pub fn handle_key(app: &mut App, key: KeyEvent) {
        match app.register_frame.selected {
            LoginSelectedBox::Username => {
                UsernameBox::handle_key(app, key);
            }
            LoginSelectedBox::Name => {
                NameBox::handle_key(app, key);
            }
            LoginSelectedBox::Password => {
                PasswordBox::handle_key(app, key);
            }
            LoginSelectedBox::ActionButton => {
                RegisterButtonBox::handle_key(app, key);
            }
            LoginSelectedBox::BackButton => {
                InitialButtonBox::handle_key(app, key, AppView::Initial);
                match key.code {
                    KeyCode::Up => {
                        app.register_frame.selected = LoginSelectedBox::ActionButton;
                        app.register_frame.back_button.selected = false;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let main = Block::default()
            .border_style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL);

        f.render_widget(main, area);

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(1),
                    Constraint::Percentage(38),
                    Constraint::Percentage(1),
                    Constraint::Percentage(30),
                ]
                .as_ref(),
            )
            .split(area);

        let row_constraints = [
            Constraint::Percentage(10),
            Constraint::Percentage(7),
            Constraint::Percentage(7),
            Constraint::Percentage(7),
            Constraint::Percentage(2),
            Constraint::Percentage(7),
            Constraint::Percentage(2),
            Constraint::Percentage(7),
            Constraint::Percentage(2),
            Constraint::Percentage(7),
            Constraint::Percentage(44),
        ]
        .as_ref();

        let left_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(columns[2]);

        let username_area = left_rows[1];
        let name_area = left_rows[3];
        let password_area = left_rows[5];
        let action_area = left_rows[7];
        let back_area = left_rows[9];

        self.username.render(
            f,
            username_area,
            matches!(self.selected, LoginSelectedBox::Username),
        );
        self.name.render(
            f,
            name_area,
            matches!(self.selected, LoginSelectedBox::Name),
        );
        self.password.render(
            f,
            password_area,
            matches!(self.selected, LoginSelectedBox::Password),
        );

        self.register_button.render(
            f,
            action_area,
            matches!(self.selected, LoginSelectedBox::ActionButton),
        );

        self.back_button.render(f, back_area);
    }
}

pub struct UsernameBox {
    text: String,
    locked: bool,
}

impl UsernameBox {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            locked: false,
        }
    }

    pub fn add_char(&mut self, c: char) {
        self.text.push(c);
    }

    pub fn del_char(&mut self) {
        self.text.pop();
    }

    pub fn handle_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Down => {
                if !app.register_frame.username.locked {
                    app.register_frame.selected = LoginSelectedBox::Name;
                }
            }
            KeyCode::Enter => {
                app.register_frame.username.locked = true;
            }
            KeyCode::Esc => {
                app.register_frame.username.locked = false;
            }
            KeyCode::Backspace => {
                if app.register_frame.username.locked {
                    app.register_frame.username.del_char();
                }
            }
            KeyCode::Char(c) => {
                if app.register_frame.username.locked {
                    app.register_frame.username.add_char(c);
                }
            }
            _ => {}
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect, is_selected: bool) {
        let border_style = get_border_style(is_selected, self.locked);

        let mut text = vec![Span::styled(
            self.text.clone(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )];

        if self.locked {
            let cursor = Span::styled("█", Style::default().fg(Color::White));

            text.push(cursor);
        }

        let username = Paragraph::new(Spans::from(Spans::from(text)))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title("Email"),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(username, area);
    }
}

pub struct NameBox {
    text: String,
    locked: bool,
}

impl NameBox {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            locked: false,
        }
    }

    pub fn add_char(&mut self, c: char) {
        self.text.push(c);
    }

    pub fn del_char(&mut self) {
        self.text.pop();
    }

    pub fn handle_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Down => {
                if !app.register_frame.name.locked {
                    app.register_frame.selected = LoginSelectedBox::Password;
                }
            }
            KeyCode::Up => {
                if !app.register_frame.name.locked {
                    app.register_frame.selected = LoginSelectedBox::Username;
                }
            }
            KeyCode::Enter => {
                app.register_frame.name.locked = true;
            }
            KeyCode::Esc => {
                app.register_frame.name.locked = false;
            }
            KeyCode::Backspace => {
                if app.register_frame.name.locked {
                    app.register_frame.name.del_char();
                }
            }
            KeyCode::Char(c) => {
                if app.register_frame.name.locked {
                    app.register_frame.name.add_char(c);
                }
            }
            _ => {}
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect, is_selected: bool) {
        let border_style = get_border_style(is_selected, self.locked);

        let mut text = vec![Span::styled(
            self.text.clone(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )];

        if self.locked {
            let cursor = Span::styled("█", Style::default().fg(Color::White));

            text.push(cursor);
        }

        let username = Paragraph::new(Spans::from(Spans::from(text)))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title("Name"),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(username, area);
    }
}

pub struct PasswordBox {
    text: String,
    locked: bool,
}

impl PasswordBox {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            locked: false,
        }
    }

    pub fn add_char(&mut self, c: char) {
        self.text.push(c);
    }

    pub fn del_char(&mut self) {
        self.text.pop();
    }

    pub fn handle_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Down => {
                if !app.register_frame.password.locked {
                    app.register_frame.selected = LoginSelectedBox::ActionButton;
                }
            }
            KeyCode::Up => {
                if !app.register_frame.password.locked {
                    app.register_frame.selected = LoginSelectedBox::Name;
                }
            }
            KeyCode::Enter => {
                app.register_frame.password.locked = true;
            }
            KeyCode::Esc => {
                app.register_frame.password.locked = false;
            }
            KeyCode::Backspace => {
                if app.register_frame.password.locked {
                    app.register_frame.password.del_char();
                }
            }
            KeyCode::Char(c) => {
                if app.register_frame.password.locked {
                    app.register_frame.password.add_char(c);
                }
            }
            _ => {}
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect, is_selected: bool) {
        let border_style = get_border_style(is_selected, self.locked);

        let mut text = vec![Span::styled(
            self.text.clone(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )];

        if self.locked {
            let cursor = Span::styled("█", Style::default().fg(Color::White));

            text.push(cursor);
        }

        let password = Paragraph::new(Spans::from(Spans::from(text)))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title("Password"),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(password, area);
    }
}

pub struct RegisterButtonBox {}

impl RegisterButtonBox {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                let email = std::mem::take(&mut app.register_frame.username.text);
                let mut password = std::mem::take(&mut app.register_frame.password.text);
                let name = std::mem::take(&mut app.register_frame.name.text);

                use st_read::schema::users::dsl as users_dsl;
                let connection = st_read::establish_connection();
                let hash = st_read::util::hash(&password, &name);

                let user = st_read::models::NewUser {
                    email,
                    name,
                    password_hash: hash.to_vec(),
                };
                zeroize::Zeroize::zeroize(&mut password);
                let res: Result<st_read::models::User, _> =
                    user.insert_into(users_dsl::users).get_result(&connection);
                app.register_frame.selected = LoginSelectedBox::Username;

                match res {
                    Ok(user) => {
                        app.set_user(user);
                        app.set_view(AppView::Homepage);
                    }
                    Err(_) => {}
                }
            }
            KeyCode::Up => {
                app.register_frame.selected = LoginSelectedBox::Password;
            }
            KeyCode::Down => {
                app.register_frame.selected = LoginSelectedBox::BackButton;
                app.register_frame.back_button.selected = true;
            }
            _ => {}
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect, is_selected: bool) {
        let border_style = get_border_style(is_selected, false);

        let button = Paragraph::new(Spans::from(Span::styled(
            "Register",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )))
        .alignment(tui::layout::Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .border_type(tui::widgets::BorderType::Double),
        )
        .wrap(Wrap { trim: false });

        f.render_widget(button, area);
    }
}
