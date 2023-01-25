use rand::{thread_rng, Rng};

use crate::{
    entity::{Entity, EyeColor, Gender, SkinColor},
    generation::Generation,
};

#[derive(Debug, PartialEq)]
pub enum Stage {
    FindFamilies,
    BuildGenerations,
}

#[derive(Debug)]
pub struct Simulator {
    pub peoples: Vec<Entity>,
    pub recursion_limit: usize,
    pub families: Vec<Generation>,
    pub stage: Stage,
}

pub fn extract_two(f1: &[Entity], f2: &[Entity]) -> Option<(Vec<Entity>, Vec<Entity>)> {
    let fem1 = f1.iter().find(|x| x.gender == Gender::Female)?.to_owned();
    let fem2 = f2.iter().find(|x| x.gender == Gender::Female)?.to_owned();
    let male1 = f1.iter().find(|x| x.gender == Gender::Male)?.to_owned();
    let male2 = f2.iter().find(|x| x.gender == Gender::Male)?.to_owned();

    Some((vec![fem1, male2], vec![male1, fem2]))
}

pub fn merge_eyes(a: EyeColor, b: EyeColor) -> EyeColor {
    use EyeColor::{Blue, Brown, Green, Hazel};
    let mut rng = thread_rng();
    // Brown (2P):
    // 75% -> Brown
    // 18.8% -> Green
    // 6.3% Blue
    // Blue (2P)
    // 99% -> Blue
    // 1% -> Green
    // 0% -> Brown
    // Green (2P)
    // 75% Green
    // 25% Blue
    // 0% -> Brown

    match (a, b) {
        (Brown, Brown) => match rng.gen_range(0..100) {
            0..=74 => Brown,
            75..=93 => Green,
            94..=100 => Blue,
            _ => unreachable!(),
        },

        (Blue, Blue) => match rng.gen_range(0..100) {
            0..=98 => Blue,
            99..=100 => Green,
            _ => unreachable!(),
        },

        (Green, Green) => match rng.gen_range(0..100) {
            0..=74 => Green,
            75..=100 => Blue,
            _ => unreachable!(),
        },

        (Brown, Blue) | (Blue, Brown) => match rng.gen_range(0..100) {
            0..=48 => Brown,
            49..=100 => Blue,
            _ => unreachable!(),
        },

        (Blue, Green) | (Green, Blue) => match rng.gen_range(0..100) {
            0..=48 => Green,
            49..=100 => Blue,
            _ => unreachable!(),
        },

        (Brown, Green) | (Green, Brown) => match rng.gen_range(0..100) {
            0..=49 => Brown,
            50..=86 => Green,
            87..=100 => Blue,
            _ => unreachable!(),
        },

        (a, _) => {
            if rng.gen_bool(0.5) {
                Hazel
            } else {
                a
            }
        }
    }
}

pub fn merge_skin(a: SkinColor, b: SkinColor) -> SkinColor {
    use SkinColor::{Black, Dark, White};

    match (a, b) {
        (White, Black) | (Black, White) => Dark,
        (White, White) => White,
        (Dark, Dark) => Dark,
        (Dark, White) | (White, Dark) => White,
        (Dark, Black) | (Black, Dark) | (Black, Black) => Black,
    }
}

pub fn merge_pep(a: &Entity, b: &Entity) -> Entity {
    Entity {
        eye_color: merge_eyes(a.eye_color, b.eye_color),
        skin_color: merge_skin(a.skin_color, b.skin_color),
        gender: rand::random(),
    }
}

impl Simulator {
    pub fn new(peoples: Vec<Entity>, recursion_limit: usize) -> Self {
        Self {
            peoples,
            recursion_limit,
            families: vec![],
            stage: Stage::FindFamilies,
        }
    }

    fn find_fm(&mut self) -> Option<(Entity, Entity)> {
        let mut female = None;
        let mut male = None;
        let search = self.peoples.clone();

        for p in search.into_iter() {
            let mut remove_this = |p: &Entity| {
                let position = self.peoples.iter().position(|x| x == p).unwrap();
                self.peoples.remove(position);
            };

            if p.gender == Gender::Female && female.is_none() {
                female = Some(p.clone());
                remove_this(&p);
                continue;
            }

            if p.gender == Gender::Male && male.is_none() {
                male = Some(p.clone());
                remove_this(&p);
            }
        }

        Some((female?, male?))
    }

    pub fn find_familes(&mut self) -> Option<()> {
        let (female, male) = self.find_fm()?;

        log::debug!("Found they: {female:?} {male:?}");
        let child = merge_pep(&female, &male);
        let other_child = merge_pep(&male, &female);

        let relation = Generation {
            children: Box::new([child, other_child]),
            descedent: None,
            father: male,
            mother: female,
            descedents_count: 0,
        };

        log::debug!("Created Relation: {relation:#?}");
        self.families.push(relation);

        Some(())
    }

    pub fn run_interaction(&mut self) -> Option<()> {
        match self.find_familes() {
            Some(()) => return Some(()),
            None => self.stage = Stage::BuildGenerations,
        };

        if self.stage == Stage::BuildGenerations {
            let mut new_families = vec![];

            for chunk in self.families.chunks(2) {
                if chunk.len() < 2 {
                    log::debug!("Skipping family");
                    continue;
                }

                let mut family_a = chunk[0].clone();
                let mut family_b = chunk[1].clone();
                log::debug!("Family A: {family_a:#?}");
                log::debug!("Family B: {family_b:#?}");

                if family_a.descedents_count > self.recursion_limit
                    || family_b.descedents_count > self.recursion_limit
                {
                    log::debug!("Stopping! Family A/B has more than recursion limit.");
                    continue;
                }

                if let Some((fa, fb)) =
                    extract_two(family_a.children.as_slice(), family_b.children.as_slice())
                {
                    println!("??? {:#?}/{:#?}", fa, fb);
                    let mut sim_a = Simulator::new(fa, 1);

                    while sim_a.run_interaction().is_some() {
                        if sim_a.stage == Stage::BuildGenerations {
                            break;
                        }
                    }

                    let mut sim_b = Simulator::new(fb, 1);

                    while sim_b.run_interaction().is_some() {
                        if sim_b.stage == Stage::BuildGenerations {
                            break;
                        }
                    }

                    log::warn!("{} {}", sim_a.families.len(), sim_b.families.len());
                    if !sim_a.families.is_empty() {
                        let de_a = &sim_a.families[0];
                        family_a.descedent = Some(Box::new(de_a.to_owned()));
                    }

                    if !sim_b.families.is_empty() {
                        let de_b = &sim_b.families[0];

                        family_b.descedent = Some(Box::new(de_b.to_owned()));
                    }

                    log::debug!("Built descedents.")
                }

                new_families.push(family_a);
                new_families.push(family_b);
            }

            self.families = new_families;
        }

        None
    }
}
