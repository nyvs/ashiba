use crate::{view::View, controller::Controller};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{self, Stdout},
    time::{Duration, Instant},
};
use tui::{
    backend::{CrosstermBackend},
    Terminal,
};

pub fn run(app: impl App) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal, app, Duration::from_millis(200));

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

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: impl App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| app.ui(f, f.size()))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            match event::read()? {
                Event::FocusGained => app.handle_focus_gained(),
                Event::FocusLost => app.handle_focus_lost(),
                Event::Key(ev) => app.handle_key(ev),
                Event::Mouse(ev) => app.handle_mouse(ev),
                Event::Paste(data) => app.handle_paste(data),
                Event::Resize(x, y) => app.handle_resize(x, y),
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.should_quit() {
            return Ok(());
        }
    }
}

pub trait App: Controller + View {}

impl<T> App for T
where T: Controller + View {}
