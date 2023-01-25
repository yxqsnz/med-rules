use crate::entity::Entity;

#[derive(Debug, Clone)]
pub struct Generation {
    pub mother: Entity,
    pub father: Entity,
    pub children: Box<[Entity; 2]>,
    pub descedent: Option<Box<Generation>>,
    pub descedents_count: usize,
}
