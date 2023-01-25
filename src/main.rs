use log::LevelFilter;
use med_rules::{simulator::Simulator, util::generate_base};

fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .parse_default_env()
        .init();

    let peps = generate_base();
    let mut sim = Simulator::new(peps, 2);

    while sim.run_interaction().is_some() {
        println!("{sim:#?}");
    }

    // println!("{:#?}", sim.families);
}
