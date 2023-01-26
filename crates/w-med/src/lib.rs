use med_rules::{simulator::Simulator, util::generate_base};
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Serialize)]
pub struct JSResponse {
    pub total_generations: usize,
    pub simulator: Simulator,
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    log::info!("WASM Setup Done")
}

#[wasm_bindgen]
pub fn generate(repeat: usize, recursion_limit: usize) -> Option<String> {
    loop {
        let mut base = generate_base();
        let mut found = 0;
        for _ in 0..repeat {
            base.extend(generate_base());
        }
        let mut sim = Simulator::new(base, recursion_limit);
        while sim.run_interaction(&mut found).is_some() {}

        if found == 0 {
            continue;
        } else {
            let response = JSResponse {
                total_generations: found,
                simulator: sim,
            };

            break serde_json::to_string(&response).ok();
        }
    }
}
