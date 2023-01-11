use std::ops::{AddAssign, SubAssign};


pub struct BasicGauge<T> {
    value: T,
    max: T,
    ratio: f64,
    percent: u16
}

impl<T: Copy + AddAssign + SubAssign + PartialOrd + Default + Into<f64>> BasicGauge<T> {
    pub fn percent(&self) -> u16 {
        self.percent
    }

    pub fn ratio(&self) -> f64 {
        self.ratio
    }

    fn set_ratio(value: &T, max: &T) -> f64 {
        let r: f64 = (*value).into() / (*max).into();
        r.min(1.)
    }

    fn set_percent(value: &T, max: &T) -> u16 {
        let p: u16 = (((*value).into() / (*max).into()) * 100.) as u16;
        p.min(100)
    }

    pub fn new(value: T, max: T) -> Self {
        let ratio = BasicGauge::set_ratio(&value, &max);
        let percent = BasicGauge::set_percent(&value, &max);
        Self {
            value,
            max,
            ratio,
            percent,
        }
    }

    pub fn set_value(&mut self, value: T) {
        self.ratio = BasicGauge::set_ratio(&value, &self.max);
        self.percent = BasicGauge::set_percent(&value, &self.max);
    }

    pub fn add_value(&mut self, value: T) {
        self.value += value;
        if self.value > self.max {
            self.value = self.max;
        }
        self.set_value(self.value);
    }

    pub fn remove_value(&mut self, value: T) {
        self.value -= value;
        if self.value < T::default() {
            self.value = T::default();
        }
        self.set_value(self.value);
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}
