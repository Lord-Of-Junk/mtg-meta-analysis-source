// This file defines all of the functions required to generate a confidence interval for the
// probability of player 1 winning a game given two magnitudes; player 1's and player 2's
use game::Game;
use provider::Provider;

// The value for t* we can use for a 95% CI
// Noted on page 360 of Discrete-Event Simulation: A First Course
const T: f64 = 1.960;

fn should_continue_interval(n: f64, v: f64, w: f64) -> bool {
    n < 40.0 || ( T * ( v / n ).sqrt() > w * ( n - 1.0 ).sqrt() )
}

fn generate(game: &mut Game) -> f64 {
    match game.run() {
        true  => 1.0,
        false => 0.0,
    }
}

pub fn generate_interval(p1: usize, p2: usize, w: f64, prov: &mut Provider<f64>) -> f64 {
    let mut g = Game::new(prov, p1, p2); 
    
    // This is a more or less direct implementation of algorithm 8.1.2 on page 361
    // of Discrete-Event Simulation: A First Course
    let mut x = generate(&mut g);
    let mut n = 1.0;
    let mut v = 0.0;
    let mut p_hat = x.clone();
    while should_continue_interval(n.clone(), v.clone(), w.clone()) {
        // We want the game to be ready to run another
        g.reset();

        // Book keeping
        x = generate(&mut g);
        n += 1.0;
        g.reset();
        let d = x - p_hat;
        v = v + d * d * ( n - 1.0 ) / n;
        p_hat = p_hat + d / n;
    }

    p_hat

}
