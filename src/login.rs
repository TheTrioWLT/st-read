use crossterm::event::{KeyCode, KeyEvent};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Borders, Paragraph, Wrap};
use tui::Frame;
use tui::{backend::Backend, widgets::Block};

use crate::app::{get_border_style, App, AppView};
use crate::initial::{InitialButtonBox, LoginSelectedBox};

pub struct LoginFrame {
    username: UsernameBox,
    password: PasswordBox,
    login_button: LoginButtonBox,
    back_button: InitialButtonBox,
    selected: LoginSelectedBox,
}

impl LoginFrame {
    pub fn new() -> Self {
        Self {
            username: UsernameBox::new(),
            password: PasswordBox::new(),
            login_button: LoginButtonBox::new(),
            back_button: InitialButtonBox::new("Back", false),
            selected: LoginSelectedBox::Username,
        }
    }

    pub fn handle_key(app: &mut App, key: KeyEvent) {
        match app.login_frame.selected {
            LoginSelectedBox::Username => {
                UsernameBox::handle_key(app, key);
            }
            LoginSelectedBox::Password => {
                PasswordBox::handle_key(app, key);
            }
            LoginSelectedBox::ActionButton => {
                LoginButtonBox::handle_key(app, key);
            }
            LoginSelectedBox::BackButton => {
                InitialButtonBox::handle_key(app, key, AppView::Initial);
                match key.code {
                    KeyCode::Up => {
                        app.login_frame.selected = LoginSelectedBox::ActionButton;
                        app.login_frame.back_button.selected = false;
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
                    Constraint::Percentage(15),
                    Constraint::Percentage(10),
                    Constraint::Percentage(15),
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
            Constraint::Percentage(53),
        ]
        .as_ref();

        let left_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(columns[2]);

        let username_area = left_rows[1];
        let password_area = left_rows[3];
        let action_area = left_rows[5];
        let back_area = left_rows[7];

        self.username.render(
            f,
            username_area,
            matches!(self.selected, LoginSelectedBox::Username),
        );
        self.password.render(
            f,
            password_area,
            matches!(self.selected, LoginSelectedBox::Password),
        );

        self.login_button.render(
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
                if !app.login_frame.username.locked {
                    app.login_frame.selected = LoginSelectedBox::Password;
                }
            }
            KeyCode::Enter => {
                app.login_frame.username.locked = true;
            }
            KeyCode::Esc => {
                app.login_frame.username.locked = false;
            }
            KeyCode::Backspace => {
                if app.login_frame.username.locked {
                    app.login_frame.username.del_char();
                }
            }
            KeyCode::Char(c) => {
                if app.login_frame.username.locked {
                    app.login_frame.username.add_char(c);
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
                    .title("Username"),
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
                if !app.login_frame.password.locked {
                    app.login_frame.selected = LoginSelectedBox::ActionButton;
                }
            }
            KeyCode::Up => {
                if !app.login_frame.password.locked {
                    app.login_frame.selected = LoginSelectedBox::Username;
                }
            }
            KeyCode::Enter => {
                app.login_frame.password.locked = true;
            }
            KeyCode::Esc => {
                app.login_frame.password.locked = false;
            }
            KeyCode::Backspace => {
                if app.login_frame.password.locked {
                    app.login_frame.password.del_char();
                }
            }
            KeyCode::Char(c) => {
                if app.login_frame.password.locked {
                    app.login_frame.password.add_char(c);
                }
            }
            _ => {}
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect, is_selected: bool) {
        let border_style = get_border_style(is_selected, self.locked);

        let mut stars = String::new();

        for _ in self.text.chars() {
            stars.push('*');
        }

        let text = vec![Span::styled(
            stars,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )];

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

pub struct LoginButtonBox {}

impl LoginButtonBox {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                app.set_view(AppView::Homepage);
            }
            KeyCode::Up => {
                app.login_frame.selected = LoginSelectedBox::Password;
            }
            KeyCode::Down => {
                app.login_frame.selected = LoginSelectedBox::BackButton;
                app.login_frame.back_button.selected = true;
            }
            _ => {}
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect, is_selected: bool) {
        let border_style = get_border_style(is_selected, false);

        let button = Paragraph::new(Spans::from(Span::styled(
            "Login",
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
