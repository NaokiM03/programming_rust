pub struct Fern {
    pub size: f64,
    pub growth_rate: f64
}

impl Fern {
    pub fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

pub fn run_simularion(fern: &mut Fern, days: usize) {
    for _ in 0 .. days {
        fern.grow();
    }
}

#[cfg(test)]
mod tests {
    use super::{Fern, run_simularion};
    
    #[test]
    fn test_grow() {
        let mut fern = Fern {
            size: 1.0,
            growth_rate: 0.5
        };
        fern.grow();
        assert_eq!(fern.size, 1.5);
        fern.grow();
        assert_eq!(fern.size, 2.25);        
    }
    #[test]
    fn test_run_simulation() {
        let mut fern = Fern {
            size: 1.0,
            growth_rate: 0.5
        };
        run_simularion(&mut fern, 1);
        assert_eq!(fern.size, 1.5);
        run_simularion(&mut fern, 3);
        assert_eq!(fern.size, 5.0625);
    }
}
