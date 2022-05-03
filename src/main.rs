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
mod list;
mod posts_list;
mod viewing_post;

//use diesel::prelude::*;

static TERMINAL: AtomicPtr<Terminal<CrosstermBackend<Stdout>>> =
    AtomicPtr::new(std::ptr::null_mut());

fn main() -> Result<(), Box<dyn Error>> {
    // set vars from the `.env` file
    dotenv::dotenv().ok();

    /*
    use st_read::models::{Post, User};
    use st_read::schema::post::dsl as post_dsl;
    use st_read::schema::users::dsl as users_dsl;

    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let hash = st_read::util::hash(name.trim(), name.trim());

    println!("decode('{}', 'base64')", base64::encode(&hash));

    let connection = st_read::establish_connection();
    let mut new_post_id = 0;
    if let Ok(posts) = post_dsl::post.get_results::<Post>(&connection) {
        new_post_id = posts.len() as i32;
        println!("Posts");
        for post in posts {
            println!("{}", post.title);
            println!("----------\n");
            println!("{}", post.text);
        }
    }

    if let Ok(users) = users_dsl::users.get_results::<User>(&connection) {
        println!("Users");
        for user in users {
            println!("{:?}", user);
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

    */

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    TERMINAL.store(&mut terminal, Ordering::SeqCst);

    std::panic::set_hook(Box::new(|i| {
        println!("{}", i);
        let ptr = TERMINAL.load(Ordering::SeqCst);
        if ptr.is_null() {
            return;
        }
        //hope for the best from rustc...
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
        println!("fixed term");
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

    if let Err(err) = res {
        println!("{:?}", err)
    }
    TERMINAL.store(std::ptr::null_mut(), Ordering::SeqCst);

    Ok(())
}
