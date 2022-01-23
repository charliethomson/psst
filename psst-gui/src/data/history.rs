use druid::{im::Vector, Data, Lens};

use super::Nav;

#[derive(Clone, Data, Lens)]
pub struct History {
    pub history: Vector<Nav>,
    pub index: usize,
}
impl History {
    pub fn new() -> Self {
        Self {
            history: Vector::new(),
            index: 0,
        }
    }

    pub fn can_navigate_back(&self) -> bool {
        self.index > 0
    }

    pub fn can_navigate_forward(&self) -> bool {
        self.index < self.history.len().saturating_sub(1)
    }

    pub fn navigate<'a>(&mut self, current_nav: &'a Nav, nav: &'a Nav) -> Option<Nav> {
        if current_nav == nav {
            return None;
        }

        if self.can_navigate_forward() {
            self.history.split_off(self.index);
        }
        self.history.push_back(current_nav.clone());
        self.history.push_back(nav.clone());
        self.index += 1;

        Some(nav.clone())
    }

    pub fn navigate_back(&mut self) -> Option<Nav> {
        if self.can_navigate_back() {
            self.index -= 1;
            self.history.get(self.index).cloned()
        } else {
            None
        }
    }

    pub fn navigate_forward(&mut self) -> Option<Nav> {
        if self.can_navigate_forward() {
            self.index += 1;
            self.history.get(self.index).cloned()
        } else {
            None
        }
    }
}
