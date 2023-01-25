use crate::entity::{Entity, Gender};

#[must_use] pub fn fm_stats(peps: &[Entity]) -> (usize, usize) {
    let mut fem_count = 0;
    let mut male_count = 0;

    for p in peps {
        if p.gender == Gender::Female {
            fem_count += 1;
        } else {
            male_count += 1;
        }
    }
    (
        (fem_count * 100 / peps.len()),
        (male_count * 100 / peps.len()),
    )
}

#[must_use] pub fn generate_base() -> Vec<Entity> {
    let mut peps = Vec::with_capacity(10);

    loop {
        for _ in 0..10 {
            peps.push(Entity::random())
        }

        let (fm, ml) = fm_stats(&peps);

        if fm == 50 && ml == 50 {
            break;
        } else {
            peps.clear()
        }
    }

    log::debug!("Peoples: {peps:#?}");

    let (fm, ml) = fm_stats(&peps);
    log::debug!("Status: {fm}% are female, {ml}% are male");

    peps
}
