use crossterm::event::{KeyCode, KeyEvent};
use tui::backend::Backend;
use tui::layout::Constraint;
use tui::layout::Direction;
use tui::layout::Layout;
use tui::layout::Rect;
use tui::style::Color;
use tui::style::Modifier;
use tui::style::Style;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::Block;
use tui::widgets::Borders;
use tui::widgets::Paragraph;
use tui::Frame;

use crate::app::get_border_style;
use crate::app::App;
use crate::app::AppView;
use crate::app::Post;
use crate::app::SelectedFrame;

#[derive(Debug, Clone, Copy)]
enum SelectedBox {
    Main,
    Comments,
    Reply,
}

pub struct ViewingPostFrame {
    post: Option<Post>,
    selected_box: SelectedBox,
    comment_box: CommentBox,
    reply_box: ReplyBox,
    /// A boolean for if we are "locked" into the "PostBox" section so that we can scroll through
    /// the post. Esc must be pressed to un-lock
    locked: bool,
}

impl ViewingPostFrame {
    pub fn new() -> Self {
        Self {
            post: None,
            selected_box: SelectedBox::Main,
            comment_box: CommentBox::new(),
            reply_box: ReplyBox::new(),
            locked: false,
        }
    }

    pub fn view(&mut self, post: Post) {
        self.comment_box.clear();
        for comment in &post.comments {
            self.comment_box.add_comment(comment.clone());
        }

        self.post = Some(post);

        self.reply_box.clear();
    }

    pub fn clear(&mut self) {
        self.post = None;
        self.comment_box.clear();
        self.reply_box.clear();
    }

    pub fn has_post(&self) -> bool {
        self.post.is_some()
    }

    pub fn handle_key(app: &mut App, key: KeyEvent) {
        if matches!(key.code, KeyCode::Left) {
            app.selected_frame = SelectedFrame::Posts;
        } else {
            match app.viewing_frame.selected_box {
                SelectedBox::Main => {
                    // This stuff
                    match key.code {
                        KeyCode::Enter => {
                            // Lock this box
                            app.viewing_frame.locked = true;
                        }
                        KeyCode::Esc => {
                            // Unlock the box, or exit the view
                            if app.viewing_frame.locked {
                                app.viewing_frame.locked = false;
                            } else {
                                app.selected_frame = SelectedFrame::Posts;
                                app.viewing_frame.clear();
                            }
                        }
                        KeyCode::Down => {
                            if app.viewing_frame.locked {
                                // Scroll through the post
                            } else {
                                // Move down to the CommentBox
                                app.viewing_frame.selected_box = SelectedBox::Comments;
                            }
                        }
                        KeyCode::Char('p') => {
                            if !app.viewing_frame.locked {
                                app.set_view(AppView::UserProfile);
                            }
                        }
                        KeyCode::Char('c') => {
                            if !app.viewing_frame.locked {
                                app.set_view(AppView::CreatePost);
                            }
                        }
                        _ => {}
                    }
                }
                SelectedBox::Comments => {
                    CommentBox::handle_key(app, key);
                }
                SelectedBox::Reply => {
                    ReplyBox::handle_key(app, key);
                }
            }
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect, is_selected: bool) {
        if let Some(post) = self.post.clone() {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(40),
                        Constraint::Percentage(40),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(area);
            let main_area = layout[0];
            let comments_area = layout[1];
            let reply_area = layout[2];

            let mut text = Vec::new();

            text.push(Spans::from(Span::styled(
                post.title,
                Style::default().add_modifier(Modifier::BOLD),
            )));

            text.push(Spans::from(Span::styled(
                format!("   by {}", post.author),
                Style::default().add_modifier(Modifier::ITALIC),
            )));

            let text_strs = format!("\n{}\n\n", post.full);

            for line in text_strs.split('\n') {
                text.push(Spans::from(vec![Span::raw(line)]));
            }

            let border_style = get_border_style(
                is_selected && matches!(self.selected_box, SelectedBox::Main),
                self.locked,
            );

            let post = Paragraph::new(text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(border_style)
                        .title("Viewing Post"),
                )
                .style(Style::default().fg(Color::Gray))
                .wrap(tui::widgets::Wrap { trim: false });

            f.render_widget(post, main_area);

            self.comment_box.render(
                f,
                comments_area,
                is_selected && matches!(self.selected_box, SelectedBox::Comments),
            );

            self.reply_box.render(
                f,
                reply_area,
                is_selected && matches!(self.selected_box, SelectedBox::Reply),
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct Comment {
    pub text: String,
    pub author: String,
    pub children: Vec<Comment>,
}

impl Comment {
    pub fn new(text: impl AsRef<str>, author: impl AsRef<str>, children: Vec<Comment>) -> Self {
        Self {
            text: String::from(text.as_ref()),
            author: String::from(author.as_ref()),
            children,
        }
    }
}

pub struct CommentBox {
    comments: Vec<Comment>,
    locked: bool,
}

impl CommentBox {
    pub fn new() -> Self {
        Self {
            comments: Vec::new(),
            locked: false,
        }
    }

    pub fn add_comment(&mut self, comment: Comment) {
        self.comments.push(comment);
    }

    pub fn clear(&mut self) {
        self.comments.clear();
    }

    pub fn handle_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                if app.viewing_frame.comment_box.locked {
                    app.viewing_frame.comment_box.locked = false;
                } else {
                    app.selected_frame = SelectedFrame::Posts;
                    app.viewing_frame.clear();
                }
            }
            KeyCode::Enter => {
                app.viewing_frame.comment_box.locked = true;
            }
            KeyCode::Down => {
                if app.viewing_frame.comment_box.locked {
                } else {
                    app.viewing_frame.selected_box = SelectedBox::Reply;
                }
            }
            KeyCode::Up => {
                if app.viewing_frame.comment_box.locked {
                } else {
                    app.viewing_frame.selected_box = SelectedBox::Main;
                }
            }
            KeyCode::Char('p') => {
                if !app.viewing_frame.comment_box.locked {
                    app.set_view(AppView::UserProfile);
                }
            }
            KeyCode::Char('c') => {
                if !app.viewing_frame.comment_box.locked {
                    app.set_view(AppView::CreatePost);
                }
            }
            _ => {}
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect, is_selected: bool) {
        let border_style = get_border_style(is_selected, self.locked);

        let mut spans = Vec::new();

        for comment in &self.comments {
            spans.push(Spans::from(Span::styled(
                comment.author.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            )));

            for line in comment.text.lines() {
                spans.push(Spans::from(Span::raw(format!("   {}", line))));
            }
        }

        let comments = Paragraph::new(spans)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title("Comments"),
            )
            .style(Style::default().fg(Color::Gray))
            .wrap(tui::widgets::Wrap { trim: false });

        f.render_widget(comments, area);
    }
}

pub struct ReplyBox {
    text: String,
    locked: bool,
}

impl ReplyBox {
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

    pub fn clear(&mut self) {
        self.text.clear();
    }

    pub fn handle_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                if app.viewing_frame.reply_box.locked {
                    app.viewing_frame.reply_box.locked = false;
                    app.quittable = true;
                } else {
                    app.selected_frame = SelectedFrame::Posts;
                    app.viewing_frame.clear();
                }
            }
            KeyCode::Enter => {
                if !app.viewing_frame.reply_box.locked {
                    app.viewing_frame.reply_box.locked = true;
                    app.quittable = false;
                } else {
                    app.viewing_frame.reply_box.add_char('\n');
                }
            }
            KeyCode::Up => {
                if app.viewing_frame.reply_box.locked {
                } else {
                    app.viewing_frame.selected_box = SelectedBox::Comments;
                }
            }
            KeyCode::Backspace => {
                if app.viewing_frame.reply_box.locked {
                    app.viewing_frame.reply_box.del_char();
                }
            }
            KeyCode::Char('p') if !app.viewing_frame.reply_box.locked => {
                app.set_view(AppView::UserProfile);
            }
            KeyCode::Char('c') if !app.viewing_frame.reply_box.locked => {
                app.set_view(AppView::CreatePost);
            }
            KeyCode::Char(c) => {
                if app.viewing_frame.reply_box.locked {
                    app.viewing_frame.reply_box.add_char(c);
                }
            }
            _ => {}
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect, is_selected: bool) {
        let border_style = get_border_style(is_selected, self.locked);

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

        let reply = Paragraph::new(spans)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title("Reply"),
            )
            .style(Style::default().fg(Color::Gray))
            .wrap(tui::widgets::Wrap { trim: false });

        f.render_widget(reply, area);
    }
}
