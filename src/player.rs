use provider::Provider;

// This struct is designed to hold all the information about a player that 
// we care about and need to track in the simulation.
pub struct Player {
    magnitude: usize,       // The "magnitude" of the player's deck, as defined in the docs
    creatures: [usize; 2],       // The number of creatures the player has out on the field. The first indicates summoning sickness.
    health: usize,          // The health remaining. Once this hits zero: X_X
    land: usize,            // The number of lands our player has on the battle field
    hand: [usize; 2],   // The number of (creature, land) cards our player has in HAND
    deck: [usize; 2],   // The number of (creature, land) cards our player has in DECK
}

impl Player {

    // Return a fresh new player to use
    pub fn new(n: usize) -> Player {
        Player {
            magnitude: n,
            creatures: [0,0],
            health: 20,
            land: 0,
            hand: [0, 0],
            deck: [20, 20],
        }
    }

    // Reset the player to their default state
    pub fn reset(&mut self) {
        self.creatures = [0, 0];
        self.health = 20;
        self.land = 0;
        self.hand = [0, 0];
        self.deck = [20, 20];
    }

    // If we draw, return Some(b) where b is a boolean indicating if the card was a land
    pub fn draw(&mut self, p: &mut Provider<f64>) -> Option<bool> {
        if self.count_deck() > 0 { 
            let card_type = if p.next().unwrap() < self.deck[1] as f64 / self.count_deck() as f64 { 1 } else { 0 };
            self.hand[card_type] += 1;
            self.deck[card_type] -= 1;
            return Some(card_type == 1)
        }
        None
    }

    pub fn _count_hand(&self) -> usize {
        self.hand[0] + self.hand[1]
    }

    pub fn count_deck(&self) -> usize {
        self.deck[0] + self.deck[1]
    }

    // Returns an optional bool. Some(true) -> We won, Some(false) -> We lost, None -> Life goes on
    pub fn turn(&mut self, opponent: &mut Player, p: &mut Provider<f64>) -> Option<bool> {
        // println!("Beginning player's turn, Magnitude = {}", self.magnitude);
        // println!("\tCreatures: {}, {}\n\tLand: {}\n\tHealth: {}\n\tHand: {}, {}\n\tDeck: {}, {}",
        //         self.creatures[0], self.creatures[1],
        //         self.land,
        //         self.health,
        //         self.hand[0], self.hand[1],
        //         self.deck[0], self.hand[1]
        //         );
        // Untap phase/fix summoning sickness
        self.creatures[1] += self.creatures[0];
        self.creatures[0] = 0;

        // println!("\tDrawing card...");
        // Draw phase
        match self.draw(p) {
            None => {
                // println!("\tDecked myself! X_X");
                return Some(false);
            },
            _    => ()
        }

        // First Main Phase
        // Play a land if we can
        if self.hand[1] > 0 {
            self.hand[1] -= 1;
            self.land += 1;
        }

        // Combat Phase
        let damage = self.magnitude * self.creatures[1]; 
        // println!("\tDealing {} damage to our opponent", damage);
        if opponent.take_damage(damage) == 0 {
            return Some(true)
        }

        // Second Main Phase
        let mut mana_left = self.land.clone();
        // println!("\tI have {} mana to play use!", mana_left);
        // println!("\tCan I play a creature? {} && {}", self.hand[0] > 0, mana_left >= self.magnitude);
        while self.hand[0] > 0 && mana_left >= self.magnitude {
            // println!("\tPlaying a {0}/{0} for {0}!", self.magnitude);
            self.hand[0] -= 1;
            self.creatures[0] += 1;
            mana_left -= self.magnitude;
        }

        // End Phase
        None
    }

    // Take damage, returning how much health we have after that damage
    pub fn take_damage(&mut self, d: usize) -> usize {
        if d > self.health {
            return 0;
        }
        self.health -= d;
        self.health.clone()
    }

}

#[cfg(test)]
mod player_tests {

    use player::Player;
    use provider::TestProvider;

    // Test that we can get accurate counts.
    #[test]
    fn test_counts() {
        let mut t_p = TestProvider::new(vec![0.5, 0.5, 0.5]);
        let mut test_player = Player::new(2);
        assert_eq!(0, test_player.count_hand());
        assert_eq!(40, test_player.count_deck());
        test_player.draw(&mut t_p);
        assert_eq!(1, test_player.count_hand());
        assert_eq!(39, test_player.count_deck());
    }
}
