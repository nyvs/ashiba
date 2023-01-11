use core::fmt::Debug;

#[derive(Default)]
pub struct Cursor<S> {
    state: S,
    history: Vec<S>
}

impl<S> Cursor<S> 
where
    S: Default + Clone + Debug {
    pub fn add_action<A>(&mut self, action: A)
        where A: CursorAction<S> {
        if let Some(new_state) = action.transition(&self.state) {
            self.state = new_state;
            self.history.push(self.state.clone());
        }
    }

    pub fn state(&self) -> &S {
        &self.state
    }
}

pub trait CursorAction<S> {
    fn transition(&self, state: &S) -> Option<S>;
}
