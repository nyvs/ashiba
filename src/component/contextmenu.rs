use std::collections::HashMap;

use crossterm::event::{MouseEventKind, MouseButton, MouseEvent};

use super::{popup::BasicPopup, button::BasicButton};


pub struct BasicContextMenu {
    inner: BasicPopup,
    mouse_pos: (u16, u16),
    entries: HashMap<String, BasicButton>
}

impl BasicContextMenu {
    pub fn new(entries: Vec<&str>) -> Self {
        Self {
            inner: BasicPopup::default(),
            mouse_pos: (0, 0),
            entries: HashMap::from_iter(entries.iter().map(|r| ((*r).to_owned(), BasicButton::new(None))))
        }
    }

    pub fn set_mouse_pos(&mut self, ev: MouseEvent) {
        if let MouseEventKind::Down(MouseButton::Right) = ev.kind {
            self.mouse_pos = (ev.column, ev.row);
        }
    }

    pub fn show(&mut self) {
        self.inner.show();
    }

    pub fn hide(&mut self) {
        self.inner.hide();
    }

    pub fn is_shown(&self) -> bool {
        self.inner.is_shown()
    }

    pub fn mouse_pos(&self) -> (u16, u16) {
        self.mouse_pos
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.entries.keys()
    }

    pub fn entries(&mut self) -> impl Iterator<Item = (&String, &mut BasicButton)> {
        self.entries.iter_mut()
    }

    pub fn on_click<F>(&mut self, ev: MouseEvent, entry: &str, closure: F)
    where F: FnMut() {
        if let Some((_, button)) = self.entries.iter_mut()
            .find(|(name, _)| name.as_str().eq(entry)) {
                button.on_click(ev, closure)
        }
    }
}