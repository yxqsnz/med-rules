use rand::{distributions::Standard, prelude::Distribution, Rng};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EyeColor {
    Brown,
    Green,
    Blue,
    Hazel,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SkinColor {
    Black,
    White,
    Dark,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    pub gender: Gender,
    pub eye_color: EyeColor,
    pub skin_color: SkinColor,
}

impl Entity {
    pub fn random() -> Self {
        Self {
            eye_color: rand::random(),
            skin_color: rand::random(),
            gender: rand::random(),
        }
    }
}

impl Distribution<EyeColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EyeColor {
        match rng.gen_range(0..=4) {
            0 => EyeColor::Blue,
            1 => EyeColor::Hazel,
            2 => EyeColor::Green,
            _ => EyeColor::Brown,
        }
    }
}

impl Distribution<SkinColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SkinColor {
        match rng.gen_range(0..=4) {
            0 => SkinColor::Black,
            1 => SkinColor::Dark,
            _ => SkinColor::White,
        }
    }
}

impl Distribution<Gender> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender {
        match rng.gen_range(0..2) {
            0 => Gender::Male,
            _ => Gender::Female,
        }
    }
}
