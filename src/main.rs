use app::{run_app, App};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, panic::PanicInfo, sync::atomic::AtomicPtr, time::Duration};
use std::{
    io::{self, Stdout},
    sync::atomic::Ordering,
};
use tui::{backend::CrosstermBackend, widgets::ListState, Terminal};

mod app;
mod create_post;
mod initial;
mod list;
mod login;
mod posts_list;
mod profile;
mod register;
mod viewing_post;

//use diesel::prelude::*;

static TERMINAL: AtomicPtr<Terminal<CrosstermBackend<Stdout>>> =
    AtomicPtr::new(std::ptr::null_mut());

fn main() -> Result<(), Box<dyn Error>> {
    // set vars from the `.env` file
    dotenv::dotenv().ok();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    TERMINAL.store(&mut terminal, Ordering::SeqCst);

    // If we panic (basically throw an exception), reset the terminal to avoid issues
    std::panic::set_hook(Box::new(|i| {
        let ptr = TERMINAL.load(Ordering::SeqCst);
        if ptr.is_null() {
            println!("{}", i);
            return;
        }
        // hope for the best from rustc...
        // woo everyone loves undefined behavior!
        let term = unsafe { &mut *ptr };

        //Fix terminal!
        disable_raw_mode().unwrap();
        execute!(
            term.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        term.show_cursor().unwrap();
        println!("{}", i);

        std::process::exit(1)
    }));

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

    match res {
        Err(err) => {
            println!("{:?}", err)
        }
        Ok(_app) => {
            //println!("{:#?}", app.posts_frame.posts.items);
        }
    }
    TERMINAL.store(std::ptr::null_mut(), Ordering::SeqCst);

    Ok(())
}
