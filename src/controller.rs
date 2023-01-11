use crossterm::event::{KeyEvent, MouseEvent};

pub trait Controller {
    fn handle_focus_gained(&mut self) {}
    fn handle_focus_lost(&mut self) {}
    fn handle_key(&mut self, ev: KeyEvent);
    #[allow(unused_variables)]
    fn handle_mouse(&mut self, ev: MouseEvent) {}
    #[allow(unused_variables)]
    fn handle_paste(&mut self, data: String) {}
    #[allow(unused_variables)]
    fn handle_resize(&mut self, x: u16, y: u16) {}

    fn should_quit(&self) -> bool { false }
    fn on_tick(&mut self) {}
}
