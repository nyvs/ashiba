#[derive(Default)]
pub struct BasicPopup {
    shown: bool
}

impl BasicPopup {
    pub fn show(&mut self) {
        self.shown = true;
    }

    pub fn hide(&mut self) {
        self.shown = false;
    }

    pub fn is_shown(&self) -> bool {
        self.shown
    }
}
