use crossterm::event::{KeyCode, KeyEvent};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Wrap};
use tui::Frame;
use tui::{backend::Backend, widgets::Paragraph};

use crate::app::{get_border_style, App, AppView};

#[derive(Debug, Copy, Clone)]
enum SelectedBox {
    Title,
    Text,
    PostButton,
}

pub struct CreatePostFrame {
    selected: SelectedBox,
    title_box: TitleBox,
    text_box: TextBox,
    post_button: PostButton,
    exitable: bool,
}

impl CreatePostFrame {
    pub fn new() -> Self {
        Self {
            selected: SelectedBox::Title,
            title_box: TitleBox::new(),
            text_box: TextBox::new(),
            post_button: PostButton::new(),
            exitable: true,
        }
    }

    pub fn handle_key(app: &mut App, key: KeyEvent) {
        if app.create_frame.exitable && matches!(key.code, KeyCode::Esc) {
            app.set_view(AppView::Homepage);
        } else {
            if matches!(app.create_frame.selected, SelectedBox::Title) {
                TitleBox::handle_key(app, key);
            } else if matches!(app.create_frame.selected, SelectedBox::Text) {
                TextBox::handle_key(app, key);
            } else if matches!(app.create_frame.selected, SelectedBox::PostButton) {
                PostButton::handle_key(app, key);
            }
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(7),
                    Constraint::Percentage(86),
                    Constraint::Percentage(7),
                ]
                .as_ref(),
            )
            .split(area);

        let title_area = layout[0];
        let text_area = layout[1];
        let button_area = layout[2];

        self.title_box
            .render(f, title_area, matches!(self.selected, SelectedBox::Title));

        self.text_box
            .render(f, text_area, matches!(self.selected, SelectedBox::Text));

        self.post_button.render(
            f,
            button_area,
            matches!(self.selected, SelectedBox::PostButton),
        );
    }
}

struct TitleBox {
    text: String,
    locked: bool,
}

impl TitleBox {
    pub fn new() -> Self {
        Self {
            text: String::from("New Post"),
            locked: false,
        }
    }

    pub fn clear(&mut self) {
        self.text = String::from("New Post");
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
                if !app.create_frame.title_box.locked {
                    app.create_frame.selected = SelectedBox::Text;
                }
            }
            KeyCode::Enter => {
                app.create_frame.title_box.locked = true;
                app.create_frame.exitable = false;
            }
            KeyCode::Esc => {
                app.create_frame.title_box.locked = false;
                app.create_frame.exitable = true;
            }
            KeyCode::Backspace => {
                if app.create_frame.title_box.locked {
                    app.create_frame.title_box.del_char();
                }
            }
            KeyCode::Char(c) => {
                if app.create_frame.title_box.locked {
                    app.create_frame.title_box.add_char(c);
                }
            }
            _ => {}
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect, is_selected: bool) {
        let title_style = get_border_style(is_selected, self.locked);

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

        let title = Paragraph::new(Spans::from(Spans::from(text)))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(title_style)
                    .title("Title"),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(title, area);
    }
}

struct TextBox {
    text: String,
    locked: bool,
}

impl TextBox {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            locked: false,
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();
    }

    pub fn add_char(&mut self, c: char) {
        self.text.push(c);
    }

    pub fn del_char(&mut self) {
        self.text.pop();
    }

    pub fn handle_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Up => {
                if !app.create_frame.text_box.locked {
                    app.create_frame.selected = SelectedBox::Title;
                }
            }
            KeyCode::Down => {
                if !app.create_frame.text_box.locked {
                    app.create_frame.selected = SelectedBox::PostButton;
                }
            }
            KeyCode::Enter => {
                if !app.create_frame.text_box.locked {
                    app.create_frame.text_box.locked = true;
                    app.create_frame.exitable = false;
                } else {
                    app.create_frame.text_box.add_char('\n');
                }
            }
            KeyCode::Esc => {
                app.create_frame.text_box.locked = false;
                app.create_frame.exitable = true;
            }
            KeyCode::Backspace => {
                if app.create_frame.text_box.locked {
                    app.create_frame.text_box.del_char();
                }
            }
            KeyCode::Char(c) => {
                if app.create_frame.text_box.locked {
                    app.create_frame.text_box.add_char(c);
                }
            }
            _ => {}
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect, is_selected: bool) {
        let text_style = get_border_style(is_selected, self.locked);

        let cursor = Span::styled("█", Style::default().fg(Color::White));

        let mut spans = Vec::new();

        if !self.text.is_empty() {
            for line in self.text.split('\n') {
                spans.push(Spans::from(Span::raw(String::from(line))));
            }

            let mut last = spans.pop().unwrap();
            last.0.push(cursor);

            spans.push(last);
        } else {
            if self.locked && is_selected {
                spans.push(Spans::from(cursor));
            }
        }

        let text = Paragraph::new(spans)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(text_style)
                    .title("Contents"),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(text, area);
    }
}

struct PostButton {}

impl PostButton {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Up => {
                app.create_frame.selected = SelectedBox::Text;
            }
            KeyCode::Enter => {
                app.create_frame.selected = SelectedBox::Title;
                app.create_frame.text_box.clear();
                app.create_frame.title_box.clear();
                app.set_view(AppView::Homepage);
            }
            _ => {}
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect, is_selected: bool) {
        let border_style = get_border_style(is_selected, false);

        let button = Paragraph::new(Spans::from(Span::raw("Make Post")))
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
