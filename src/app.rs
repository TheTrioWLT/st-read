use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use crate::StatefulList;

#[derive(Debug, Clone, Copy)]
enum AppView {
    Homepage,
    UserProfile,
}

/// This struct holds the current state of the app.
pub struct App<'a> {
    posts: StatefulList<(
        &'a str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    )>,
    viewing_post: Option<(
        &'a str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    )>,
    current_view: AppView,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            posts: StatefulList::with_items(vec![
                (
                    "My New Cat",
                    "I just purchased a new cat named Fluffy ...",
                    "33 Comments, 152 Upvotes",
                    "Troy Neubauer",
                    "",
                    "",
                    ""
                ),
                (
                    "Why I hate Windows",
                    "I've had enough with Windows ...",
                    "91 Comments, 0 Upvotes",
                    "Luke Newcomb",
                    "I have had enough with Windows. Today my machine auto-updated to Windows 11! Deleting my entire Linux partition in the process.",
                    "As deserved for a Linux user lmao. \"I use Arch BTW.\" Get out of here.",
                    "Jeremiah Webb"
                ),
                (
                    "Hello ST-Read",
                    "Hello ST-read. Welcome to our new site! ...",
                    "12 Comments, 4 Upvotes",
                    "Troy Neubauer",
                    "",
                    "",
                    ""
                ),
            ]),
            viewing_post: None,
            current_view: AppView::Homepage
        }
    }

    // /// Rotate through the event list.
    // /// This only exists to simulate some kind of "progress"
    // fn on_tick(&mut self) {
    //     let event = self.events.remove(0);
    //     self.events.push(event);
    // }
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
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Left => app.posts.unselect(),
                    KeyCode::Down => app.posts.next(),
                    KeyCode::Char('j') => app.posts.next(),
                    KeyCode::Up => app.posts.previous(),
                    KeyCode::Char('k') => app.posts.previous(),
                    KeyCode::Right => {
                        let selected = app.posts.selected();
                        app.viewing_post = selected.map(|s| *s);
                    }
                    _ => {}
                }
            }
        }

        /*
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        */
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create two chunks with equal horizontal screen space
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(7), Constraint::Percentage(93)].as_ref())
        .split(f.size());
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(vertical[1]);

    let posts: Vec<ListItem> = app
        .posts
        .items
        .iter()
        .map(|i| {
            let mut lines = Vec::new();

            lines.push(Spans::from(Span::styled(i.0, Style::default())));
            lines.push(Spans::from(Span::styled(
                format!("{}", i.1),
                Style::default().add_modifier(Modifier::ITALIC),
            )));
            lines.push(Spans::from(Span::styled(
                "",
                Style::default().add_modifier(Modifier::ITALIC),
            )));
            lines.push(Spans::from(Span::styled(
                i.2,
                Style::default().add_modifier(Modifier::ITALIC),
            )));
            lines.push(Spans::from(Span::styled(
                format!("by {}", i.3),
                Style::default().add_modifier(Modifier::ITALIC),
            )));

            ListItem::new(lines).style(Style::default().fg(Color::Gray))
        })
        .collect();

    // Create a List from all posts and highlight the currently selected one
    let posts = List::new(posts)
        .block(Block::default().borders(Borders::ALL).title("Posts"))
        .highlight_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    if let Some(vp) = app.posts.selected() {
        let mut text = Vec::new();

        let text_strs = format!(
            "{}\nby {}\n\n{}\n\nTop Comments\n{}\n- {}",
            vp.0, vp.3, vp.4, vp.5, vp.6
        );

        for line in text_strs.split('\n') {
            text.push(Spans::from(vec![Span::raw(line)]));
        }

        let post = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Viewing Post:"),
            )
            .style(Style::default().fg(Color::Gray))
            .wrap(tui::widgets::Wrap { trim: false });

        if matches!(app.current_view, AppView::Homepage) {
            f.render_widget(post, chunks[1]);
        }
    }

    if matches!(app.current_view, AppView::Homepage) {
        // We can now render the posts list
        f.render_stateful_widget(posts, chunks[0], &mut app.posts.state);
    }

    let page_title_block = Paragraph::new("Homepage")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray))
        .wrap(tui::widgets::Wrap { trim: false });

    f.render_widget(page_title_block, vertical[0]);

    /*
    // Let's do the same for the events.
    // The event list doesn't have any state and only displays the current state of the list.
    let events: Vec<ListItem> = app
        .events
        .iter()
        .rev()
        .map(|&(event, level)| {
            // Colorcode the level depending on its type
            let s = match level {
                "CRITICAL" => Style::default().fg(Color::Red),
                "ERROR" => Style::default().fg(Color::Magenta),
                "WARNING" => Style::default().fg(Color::Yellow),
                "INFO" => Style::default().fg(Color::Blue),
                _ => Style::default(),
            };
            // Add a example datetime and apply proper spacing between them
            let header = Spans::from(vec![
                Span::styled(format!("{:<9}", level), s),
                Span::raw(" "),
                Span::styled(
                    "2020-01-01 10:00:00",
                    Style::default().add_modifier(Modifier::ITALIC),
                ),
            ]);
            // The event gets its own line
            let log = Spans::from(vec![Span::raw(event)]);

            // Here several things happen:
            // 1. Add a `---` spacing line above the final list entry
            // 2. Add the Level + datetime
            // 3. Add a spacer line
            // 4. Add the actual event
            ListItem::new(vec![
                Spans::from("-".repeat(chunks[1].width as usize)),
                header,
                Spans::from(""),
                log,
            ])
        })
        .collect();
    let events_list = List::new(events)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .start_corner(Corner::BottomLeft);
    f.render_widget(events_list, chunks[1]);
    */
}
