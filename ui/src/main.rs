use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn selected(&self) -> Option<&T> {
        let i = self.state.selected()?;
        self.items.get(i)
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
struct App<'a> {
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
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
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
        }
    }

    // /// Rotate through the event list.
    // /// This only exists to simulate some kind of "progress"
    // fn on_tick(&mut self) {
    //     let event = self.events.remove(0);
    //     self.events.push(event);
    // }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
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
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

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

    // We can now render the posts list
    f.render_stateful_widget(posts, chunks[0], &mut app.posts.state);

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

        f.render_widget(post, chunks[1]);
    }

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
