use crossterm::event::{KeyCode, KeyEvent};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::Color;
use tui::style::Modifier;
use tui::style::Style;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::Block;
use tui::widgets::Borders;
use tui::widgets::Paragraph;
use tui::widgets::Wrap;
use tui::Frame;

use crate::app::get_border_style;
use crate::app::App;
use crate::app::AppView;
use crate::app::Post;
use crate::app::SelectedFrame;
use crate::list::StatefulList;

pub struct PostsListFrame {
    pub posts: StatefulList<Post>,
}

impl PostsListFrame {
    /// Creates a new post list frame with the provided items
    pub fn with_items(posts: Vec<Post>) -> Self {
        let mut posts = StatefulList::with_items(posts);
        posts.with_highlight_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );
        posts.with_highlight_symbol(">> ");
        posts.next();

        Self { posts }
    }

    /// Handles the key that the user has pressed
    pub fn handle_key(app: &mut App, key: KeyEvent) {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => app.posts_frame.posts.next(),
            KeyCode::Up | KeyCode::Char('k') => app.posts_frame.posts.previous(),
            KeyCode::Enter => {
                if let Some(selected) = app.posts_frame.posts.selected_item() {
                    app.viewing_frame.view(selected.clone());
                }
            }
            KeyCode::Right => {
                if app.viewing_frame.has_post() {
                    app.selected_frame = SelectedFrame::ViewPost;
                }
            }
            KeyCode::Char('p') => {
                app.set_view(AppView::UserProfile);
            }
            KeyCode::Char('c') => {
                app.set_view(AppView::CreatePost);
            }
            _ => {}
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect, is_selected: bool) {
        let highlight_symbol_width = self.posts.highlight_symbol().len();
        // The placeholder used instead of the highlighting symbol to keep everything properly
        // indented
        let placeholder = format!("{: ^width$}", "", width = highlight_symbol_width);

        // The style used for posts that isn't selected
        let deselected_style = Style::default()
            .add_modifier(Modifier::ITALIC)
            .fg(Color::Gray);

        let selected_style = self.posts.highlight_style();

        let mut text = Vec::new();

        for (i, item) in self.posts.items.iter().enumerate() {
            let mut post = String::new();

            post.push_str(&format!("{}\n", item.title));

            post.push_str(&format!("{}\n", item.short));
            post.push_str(&format!("{}\n", ""));
            post.push_str(&format!("{}\n", item.stats));
            post.push_str(&format!("by {}\n", item.author));
            post.push_str("\n");

            match self.posts.selected() {
                Some(s) if s == i => {
                    // If the post is selected
                    for (i, line) in post.lines().enumerate() {
                        if i == 0 {
                            text.push(Spans::from(vec![
                                Span::styled(self.posts.highlight_symbol(), selected_style),
                                Span::styled(String::from(line), selected_style),
                            ]))
                        } else {
                            text.push(Spans::from(vec![
                                Span::raw(placeholder.clone()),
                                Span::styled(String::from(line), selected_style),
                            ]))
                        }
                    }
                }
                _ => {
                    // If the post is not selected
                    for line in post.lines() {
                        text.push(Spans::from(vec![
                            Span::raw(placeholder.clone()),
                            Span::styled(String::from(line), deselected_style),
                        ]));
                    }
                }
            }
        }

        let border_style = get_border_style(is_selected, false);

        // Render it as a "paragraph"
        let posts = Paragraph::new(text).wrap(Wrap { trim: false }).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title("Posts"),
        );

        // We can now render the posts list
        f.render_widget(posts, area);
    }
}
