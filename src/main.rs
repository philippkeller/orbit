mod analyze;
mod app;
mod audio;
mod bucket;
mod config;
mod download;
mod ipc;
mod library;
mod media;
mod model;
mod platform;
mod queue;
mod remote;
mod session;
mod stats;
mod theme;
mod ui;

use std::env;
use std::process::ExitCode;
use std::time::{Duration, Instant};

use anyhow::Result;
use ratatui::crossterm::event::{self, Event};

use app::App;

/// Target frame interval — drives progress, queue advancement, and the
/// spectrum animation (~20 fps).
const TICK: Duration = Duration::from_millis(50);

fn main() -> ExitCode {
    match run_main() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::from(1)
        }
    }
}

fn run_main() -> Result<ExitCode> {
    let mut args = env::args().skip(1);
    if let Some(flag) = args.next() {
        if flag == "--remote" {
            return run_remote(args.next().as_deref());
        }
        if flag == "--help" || flag == "-h" {
            print_usage();
            return Ok(ExitCode::SUCCESS);
        }
        eprintln!("unknown argument: {flag}");
        print_usage();
        return Ok(ExitCode::from(2));
    }

    let mut app = App::new()?;
    let mut terminal = ratatui::try_init()
        .map_err(|e| anyhow::anyhow!("Orbit needs an interactive terminal: {e}"))?;
    let result = run_tui(&mut terminal, &mut app);
    let _ = ratatui::try_restore();
    result?;
    Ok(ExitCode::SUCCESS)
}

fn run_remote(cmd: Option<&str>) -> Result<ExitCode> {
    match cmd {
        Some("delete-current") => {
            ipc::send_cmd("delete-current")?;
            Ok(ExitCode::SUCCESS)
        }
        Some("notify-now-playing") => {
            ipc::send_cmd("notify-now-playing")?;
            Ok(ExitCode::SUCCESS)
        }
        Some(other) => {
            eprintln!("unknown remote command: {other}");
            print_usage();
            Ok(ExitCode::from(2))
        }
        None => {
            eprintln!("missing remote command");
            print_usage();
            Ok(ExitCode::from(2))
        }
    }
}

fn print_usage() {
    eprintln!(
        "Usage:\n  orbit\n  orbit --remote delete-current\n  orbit --remote notify-now-playing\n  orbit --help"
    );
}

fn run_tui(terminal: &mut ratatui::DefaultTerminal, app: &mut App) -> Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        // Wait for input, but no longer than the remaining tick budget.
        let timeout = TICK.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key);
            }
        }

        if last_tick.elapsed() >= TICK {
            app.tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}
