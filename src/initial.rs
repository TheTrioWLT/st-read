use crossterm::event::{KeyCode, KeyEvent};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Borders, Paragraph, Wrap};
use tui::Frame;
use tui::{backend::Backend, widgets::Block};

use crate::app::{get_border_style, App, AppView};

pub struct InitialFrame {
    login_button: InitialButtonBox,
    register_button: InitialButtonBox,
}

impl InitialFrame {
    pub fn new() -> Self {
        Self {
            login_button: InitialButtonBox::new("Login", true),
            register_button: InitialButtonBox::new("Register", false),
        }
    }

    pub fn handle_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Left => {
                if app.initial_frame.register_button.selected {
                    app.initial_frame.register_button.selected = false;
                    app.initial_frame.login_button.selected = true;
                }
            }
            KeyCode::Right => {
                if app.initial_frame.login_button.selected {
                    app.initial_frame.login_button.selected = false;
                    app.initial_frame.register_button.selected = true;
                }
            }
            _ => {
                if app.initial_frame.register_button.selected {
                    InitialButtonBox::handle_key(app, key, AppView::Register);
                } else if app.initial_frame.login_button.selected {
                    InitialButtonBox::handle_key(app, key, AppView::Login);
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
            Constraint::Percentage(45),
            Constraint::Percentage(10),
            Constraint::Percentage(45),
        ]
        .as_ref();

        let left_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(columns[1]);

        let right_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(columns[3]);

        let login_area = left_rows[1];
        let register_area = right_rows[1];

        self.login_button.render(f, login_area);
        self.register_button.render(f, register_area);

        f.render_widget(main, area);
    }
}

pub struct InitialButtonBox {
    text: String,
    pub selected: bool,
}

impl InitialButtonBox {
    pub fn new(text: impl AsRef<str>, selected: bool) -> Self {
        Self {
            text: String::from(text.as_ref()),
            selected,
        }
    }

    pub fn handle_key(app: &mut App, key: KeyEvent, view: AppView) {
        match key.code {
            KeyCode::Enter => app.set_view(view),
            _ => {}
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let border_style = get_border_style(self.selected, false);

        let button = Paragraph::new(Spans::from(Span::styled(
            self.text.clone(),
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

#[derive(Debug, Clone, Copy)]
pub enum LoginSelectedBox {
    Username,
    Name,
    Password,
    BackButton,
    ActionButton,
}
