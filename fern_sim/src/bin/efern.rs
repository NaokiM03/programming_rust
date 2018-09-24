extern crate fern_sim;
use fern_sim::{Fern, run_simularion};

fn main() {
    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.001
    };
    run_simularion(&mut fern, 1000);
    println!("final fern size: {}", fern.size);
}