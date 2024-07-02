use std::io;

use color_eyre::{config::HookBuilder, Result};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};

pub(crate) fn install_color_eyre() -> Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();
    let panic_hook = panic_hook.into_panic_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore();
        panic_hook(panic_info);
    }));
    let eyre_hook = eyre_hook.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |error| {
        let _ = restore();
        eyre_hook(error)
    }))?;
    Ok(())
}

pub(crate) fn init() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

pub(crate) fn restore() -> Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
