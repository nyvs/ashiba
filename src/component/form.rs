use std::{collections::HashMap};

pub struct FieldDescriptor {
    pub name: &'static str,
    pub hidden: bool,
}

pub trait FormController {
    fn on_enter(&mut self, f: &Fields);
    fn fields() -> Vec<FieldDescriptor>;
    fn placeholder() -> Vec<(&'static str, &'static str)>;
}

pub struct Form<M> {
    fields: Fields,
    model: M,
    editing: bool
}

pub struct Fields {
    values: HashMap<usize, String>,
    names: HashMap<usize, String>,
    active: usize,
}

impl Fields {
    pub fn new<M: FormController>() -> Self {
        Self {
            values: HashMap::from_iter(
                M::fields()
                    .into_iter()
                    .map(|_f| "".to_owned())
                    .enumerate(),
            ),
            names: HashMap::from_iter(
                M::fields()
                    .into_iter()
                    .map(|f| f.name.to_string())
                    .enumerate(),
            ),
            active: Default::default(),
        }
    }

    pub fn by_name(&self, name: &str) -> Option<&String> {
        let result = self.names.iter().find(|r| r.1 == name);
        if let Some(field) = result {
            self.values.get(field.0)
        } else {
            None
        }
    }

    fn field_mut(&mut self, name: &str) -> Option<&mut String> {
        let result = self.names.iter_mut().find(|r| r.1 == name);
        if let Some(field) = result {
            self.values.get_mut(field.0)
        } else {
            None
        }
    }

    pub fn get(&self, k: &usize) -> Option<&String> {
        self.values.get(k)
    }

    fn get_mut(&mut self, k: &usize) -> Option<&mut String> {
        self.values.get_mut(k)
    }

    pub fn get_name(&self, k: &usize) -> Option<&String> {
        self.names.get(k)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn active(&self) -> &usize {
        &self.active
    }

    fn active_mut(&mut self) -> &mut usize {
        &mut self.active
    }
}

impl<M> Form<M>
where
    M: FormController,
{
    pub fn new(model: M) -> Self {
        Self {
            fields: Fields::new::<M>(),
            model,
            editing: Default::default()
        }
    }

    pub fn with_placeholder(mut self) -> Self {
        for field in M::placeholder().iter() {
            if let Some(v) = self.fields.field_mut(field.0) {
                *v = field.1.to_string();
            }
        }
        self
    }

    pub fn with_values(mut self, data: Vec<(&'static str, String)>) -> Self {
        for field in data.iter() {
            if let Some(v) = self.fields.field_mut(field.0) {
                *v = field.1.clone();
            }
        }
        self
    }

    pub fn data(&self) -> &M {
        &self.model
    }

    pub fn data_mut(&mut self) -> &mut M {
        &mut self.model
    }

    pub fn on_shortcut(&mut self, shortcut: char, ch: char) {
        if !self.editing && shortcut == ch {
            self.editing = true;
        }
    }

    pub fn on_char(&mut self, ch: char) {
        if self.editing {
            let active = *self.fields.active();
            match self.fields.get_mut(&active) {
                Some(field) => field.push(ch),
                None => unreachable!(),
            }
        }
    }

    pub fn on_esc(&mut self) {
        self.editing = false;
    }

    pub fn begin_edit(&mut self) {
        self.editing = true;
    }

    pub fn editing(&self) -> bool {
        self.editing
    }

    pub fn on_backspace(&mut self) {
        if self.editing {
            let active = *self.fields.active();
            match self.fields.get_mut(&active) {
                Some(field) => {
                    field.pop();
                }
                None => unreachable!(),
            }
        }
    }

    pub fn fields(&self) -> &Fields {
        &self.fields
    }

    pub fn on_tab(&mut self) {
        if self.editing {
            let new = self.fields.active() + 1;
            match new >= self.fields.len() {
                true => *self.fields.active_mut() = 0,
                false => *self.fields.active_mut() = new
            }
        }
    }

    pub fn on_enter(&mut self) {
        if self.editing {
            M::on_enter(&mut self.model, &self.fields);
        }
    }
}

impl<M> Default for Form<M>
where M: FormController + Default {
    fn default() -> Self {
        Self { 
            fields: Fields::new::<M>(), 
            model: Default::default(), 
            editing: Default::default()
        }
    }
}
