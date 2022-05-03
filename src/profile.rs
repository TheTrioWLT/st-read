use crossterm::event::{KeyCode, KeyEvent};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Borders, Paragraph};
use tui::Frame;
use tui::{backend::Backend, widgets::Block};

use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::app::{App, AppView};

enum SelectedOption {
    DarkMode,
    EmailNotifications,
    None,
}

pub struct UserProfileFrame {
    selected: SelectedOption,
    dark_mode: bool,
    email_notifications: bool,
}

impl UserProfileFrame {
    pub fn new() -> Self {
        Self {
            selected: SelectedOption::None,
            dark_mode: true,
            email_notifications: false,
        }
    }

    pub fn handle_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                app.set_view(AppView::Homepage);
            }
            KeyCode::Down => {
                if matches!(app.profile_frame.selected, SelectedOption::None) {
                    app.profile_frame.selected = SelectedOption::DarkMode;
                } else if matches!(app.profile_frame.selected, SelectedOption::DarkMode) {
                    app.profile_frame.selected = SelectedOption::EmailNotifications;
                }
            }
            KeyCode::Up => {
                if matches!(app.profile_frame.selected, SelectedOption::None) {
                    app.profile_frame.selected = SelectedOption::EmailNotifications;
                } else if matches!(
                    app.profile_frame.selected,
                    SelectedOption::EmailNotifications
                ) {
                    app.profile_frame.selected = SelectedOption::DarkMode;
                }
            }
            KeyCode::Enter => {
                if matches!(app.profile_frame.selected, SelectedOption::DarkMode) {
                    app.profile_frame.dark_mode = !app.profile_frame.dark_mode;
                } else if matches!(
                    app.profile_frame.selected,
                    SelectedOption::EmailNotifications
                ) {
                    app.profile_frame.email_notifications = !app.profile_frame.email_notifications;
                }
            }
            KeyCode::Char('c') => {
                app.set_view(AppView::CreatePost);
            }
            _ => {}
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
                    Constraint::Percentage(15),
                    Constraint::Percentage(30),
                    Constraint::Percentage(10),
                    Constraint::Percentage(30),
                    Constraint::Percentage(15),
                ]
                .as_ref(),
            )
            .split(area);

        let left_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(5),
                    Constraint::Percentage(2),
                    Constraint::Percentage(2),
                    Constraint::Percentage(2),
                    Constraint::Percentage(7),
                    Constraint::Percentage(2),
                    Constraint::Percentage(7),
                    Constraint::Percentage(58),
                ]
                .as_ref(),
            )
            .split(columns[1]);

        let user_name_row = left_rows[1];
        let email_row = left_rows[2];
        let dark_mode_row = left_rows[4];
        let email_notifs_row = left_rows[6];

        f.render_widget(main, area);

        let user_name = Paragraph::new(Spans::from(Span::styled(
            "username",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));

        f.render_widget(user_name, user_name_row);

        let email = Paragraph::new(Spans::from(Span::styled(
            "email@website.com",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));

        f.render_widget(email, email_row);

        let dark_mode_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(dark_mode_row);
        let dark_mode_row = dark_mode_layout[0];

        let (prompt_style, value_style, border_style) =
            get_styles(matches!(self.selected, SelectedOption::DarkMode));

        let dark_mode_value = if self.dark_mode { "Yes" } else { "No" };

        let dark_mode_spans = Spans::from(vec![
            Span::styled("Dark Mode: ", prompt_style),
            Span::styled(dark_mode_value, value_style),
        ]);

        let dark_mode = Paragraph::new(dark_mode_spans).block(
            Block::default()
                .border_style(border_style)
                .borders(Borders::ALL),
        );

        f.render_widget(dark_mode, dark_mode_row);

        let (prompt_style, value_style, border_style) =
            get_styles(matches!(self.selected, SelectedOption::EmailNotifications));

        let email_notifs_value = if self.email_notifications {
            "Yes"
        } else {
            "No"
        };

        let email_notifs_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(email_notifs_row);
        let email_notifs_row = email_notifs_layout[0];

        let email_notifs_spans = Spans::from(vec![
            Span::styled("Email Notifications: ", prompt_style),
            Span::styled(email_notifs_value, value_style),
        ]);

        let email_notifs = Paragraph::new(email_notifs_spans).block(
            Block::default()
                .border_style(border_style)
                .borders(Borders::ALL),
        );

        f.render_widget(email_notifs, email_notifs_row);
    }
}

fn get_styles(selected: bool) -> (Style, Style, Style) {
    if selected {
        (
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::UNDERLINED),
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        (
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::UNDERLINED),
            Style::default().fg(Color::Gray),
        )
    }
}
