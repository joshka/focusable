use button::Button;
use container::Container;
use focusable::{Focus, FocusContainer};
use label::Label;
use text_box::TextBox;
use tracing::Level;
use tracing_subscriber::{fmt::time::Uptime, EnvFilter};

mod button;
mod container;
mod label;
mod text_box;
trait Render {
    fn render(&self);
}

trait Widget: Render + Focus {
    fn boxed(self) -> Box<dyn Widget>
    where
        Self: 'static + Sized,
    {
        Box::new(self)
    }
}

fn main() {
    init_tracing();
    let mut container = Container::from_iter([
        Label::new("This is a label").boxed(),
        TextBox::new("Hello, world!").boxed(),
        Button::new("Click me!").boxed(),
    ]);
    container.focus_next();
    container.render();
    container.focus_next();
    container.render();
    container.focus_previous();
    container.render();
}

fn init_tracing() {
    let filter = EnvFilter::builder()
        .with_default_directive(Level::TRACE.into())
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_timer(Uptime::default())
        // .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ENTER)
        .init();
}
