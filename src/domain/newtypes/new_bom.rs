use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::domain::BOMChangeEvent;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NewBOM {
    pub events: Box<Vec<BOMChangeEvent>>,
}

impl Display for NewBOM {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.events)
    }
}

impl NewBOM {
    pub fn new(events: Box<Vec<BOMChangeEvent>>) -> Self {
        Self { events }
    }
}
