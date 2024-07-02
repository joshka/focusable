use std::time::Duration;

use color_eyre::Result;
use focusable::{Focus, FocusContainer};
use ratatui::{
    crossterm::event::{self, KeyCode},
    widgets::WidgetRef,
};

use crate::{form::Form, label::Label, text_box::TextBox};

mod form;
mod label;
mod terminal;
mod text_box;

trait FocusableWidget: WidgetRef + Focus {
    fn boxed(self) -> Box<dyn FocusableWidget>
    where
        Self: 'static + Sized,
    {
        Box::new(self)
    }
}

fn main() -> Result<()> {
    terminal::install_color_eyre()?;
    let mut terminal = terminal::init()?;
    let mut running = true;

    let mut form = Form::from_iter([
        Label::new("Label 1:").boxed(),
        TextBox::new("Text 1").boxed(),
        Label::new("Label 2:").boxed(),
        TextBox::new("Text 2").boxed(),
        Label::new("Label 3:").boxed(),
        TextBox::new("Text 3").boxed(),
    ]);
    form.focus_first();
    while running {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(&form, area);
        })?;
        while event::poll(Duration::ZERO)? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => running = false,
                    KeyCode::Tab => form.focus_next(),
                    KeyCode::BackTab => form.focus_previous(),
                    _ => {}
                }
            }
        }
    }
    terminal::restore()?;
    Ok(())
}
