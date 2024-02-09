// entity/traits.rs

use uuid::Uuid;

pub trait Identifiable {
    fn id(&self) -> Uuid;
}
