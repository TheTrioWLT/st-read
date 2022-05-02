use app::{run_app, App};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, time::Duration};
use tui::{backend::CrosstermBackend, widgets::ListState, Terminal};

mod app;
mod list;
mod posts_list;
mod viewing_post;

use diesel::prelude::*;
use st_read;

fn main() -> Result<(), Box<dyn Error>> {
    use st_read::models::Post;
    use st_read::schema::post::dsl as post_dsl;

    let connection = st_read::establish_connection();
    let mut new_post_id = 0;
    if let Ok(posts) = post_dsl::post.get_results::<Post>(&connection) {
        new_post_id = posts.len() as i32;
        for post in posts {
            println!("{}", post.title);
            println!("----------\n");
            println!("{}", post.text);
        }
    }

    let post = Post {
        post_id: new_post_id,
        date_posted: std::time::SystemTime::now(),
        title: "My first post".to_owned(),
        text: "Hello".to_owned(),
    };

    diesel::insert_into(post_dsl::post)
        .values(post)
        .execute(&connection)
        .unwrap();

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
