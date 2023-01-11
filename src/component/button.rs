use crossterm::event::{MouseEvent, MouseEventKind, MouseButton};
use tui::layout::Rect;

#[derive(Default)]
pub struct BasicButton {
    area: Option<Rect>
}

impl BasicButton {
    pub fn new(area: Option<Rect>) -> Self {
        Self {
            area
        }
    }

    pub fn on_click<F>(&self, ev: MouseEvent, mut closure: F)
    where F: FnMut() {
        if let MouseEventKind::Down(MouseButton::Left) = ev.kind {
            let (x, y) = (ev.column, ev.row);
            let rect = Rect { x, y, width: 1, height: 1 };
            if self.area.map(|r| r.intersects(rect)).unwrap_or_default() {
                closure();
            }
        }
    }

    pub fn set_area(&mut self, area: Rect) {
        self.area = Some(area);
    }
}
