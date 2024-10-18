use ratatui::crossterm::event::DisableMouseCapture;
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use ratatui::crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrossTermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;

    execute!(terminal.backend, LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            print_pairs(&app.pairs);
        }
    } else if let Err(e) = res {
        println!("{err:?}");
    }
    Ok(())
}

