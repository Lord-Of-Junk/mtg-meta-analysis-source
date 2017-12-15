use player::Player;
use provider::Provider;

pub struct Game<'a> {
    provider: &'a mut Provider<f64>,
    player_1: Player,
    player_2: Player,
    turn: usize,
}

impl<'a> Game<'a> {

    pub fn new(provider_borrow: &'a mut Provider<f64>, p1_magnitude: usize, p2_magnitude: usize) -> Game<'a> {
        Game {
            provider: provider_borrow,
            player_1: Player::new(p1_magnitude),
            player_2: Player::new(p2_magnitude),
            turn: 0
        }
    }

    // Reset all of the games parameters to their initial states
    pub fn reset(&mut self) {
        self.player_1.reset();
        self.player_2.reset();
        self.turn = 0;
    }

    // Returns a boolean indicating if player 1 won
    pub fn run(&mut self) -> bool {
        for _ in 0..6 {
            self.player_1.draw(self.provider);
            self.player_2.draw(self.provider);
        }
        self.player_2.draw(self.provider);
        loop {
            self.turn += 1;
            // println!("{}", {
            //    let mut out = String::new();
            //    for _ in 0..30 {
            //        out += "#";
            //    }
            //    out
            // });
            // println!("");
            // println!("Turn: {}", self.turn);
            // println!("");
            // println!("Player 1");
            match self.player_1.turn(&mut self.player_2, self.provider) {
                Some(b) => return b,
                None => ()
            };
            // println!("");
            // println!("Player 2");
            match self.player_2.turn(&mut self.player_1, self.provider) {
                Some(b) => return !b,
                None => ()
            };
            // println!("");
        }
    }
}
