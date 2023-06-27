use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Class {
    id: u32,
    gym_id: u32,
    instructor_id: u32,
    day: String,
    time: String,
    location: String,
    price: f32,
}

impl Class {
    pub fn new(
        id: u32,
        gym_id: u32,
        instructor_id: u32,
        day: String,
        time: String,
        location: String,
        price: f32,
    ) -> Self {
        Self {
            id,
            gym_id,
            instructor_id,
            day,
            time,
            location,
            price,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Gym {
    id: u32,
    name: String,
    location: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Instructor {
    id: u32,
    name: String,
}
