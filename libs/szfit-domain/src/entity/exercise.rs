use crate::entity::Id;

#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub struct Exercise {
    pub id: Id,
    pub name: String,
}

impl Exercise {
    pub fn new(id: Id, name: String) -> Self {
        Self {
            id,
            name,
        }
    }
}