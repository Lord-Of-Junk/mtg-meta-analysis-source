pub static M_VAL: i64 = 2147483647;
pub static A_VAL: i64 = 48271;
pub static Q_VAL: i64 = 44488; // Quotient of M/A
pub static R_VAL: i64 = 3399; // M mod A

// An implementation of the Lehmer Generator presented on page 54
// of "Discrete-Event Simulation: A First Course" by Lawerence M. Leemis
// and Stephen K. Park
pub struct LehmerProvider {
    x: i64, // The seed and current value of our generator
}


pub trait Provider<T> {
    fn next(&mut self) -> Option<T>; // All providers should have a function for returning a f64
}

impl Provider<f64> for LehmerProvider { 
    
    fn next(&mut self) -> Option<f64> {
        let t = A_VAL * (self.x % Q_VAL) - R_VAL * (self.x / Q_VAL);
        if t > 0i64 {
            self.x = t;
        }
        else {
            self.x = t + M_VAL;
        }
        Some(self.x as f64 / M_VAL as f64)
    }

}

impl LehmerProvider {
    
    pub fn new(seed: i64) -> LehmerProvider {
        LehmerProvider {
            x: seed,
        }
    } 
    
    pub fn next(&mut self) -> f64 {
        let t = A_VAL * (self.x % Q_VAL) - R_VAL * (self.x / Q_VAL);
        if t > 0i64 {
            self.x = t;
        }
        else {
            self.x = t + M_VAL;
        }
        self.x as f64 / M_VAL as f64
    }

    pub fn _set_seed(&mut self, seed: i64) {
        self.x = seed;
    }

    pub fn _see_state(&self) -> i64 {
        self.x.clone()
    }
}
