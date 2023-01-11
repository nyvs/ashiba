use std::io::Stdout;

use tui::{Frame, layout::Rect, backend::{CrosstermBackend}};

pub trait View {
    fn ui(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect);
}
