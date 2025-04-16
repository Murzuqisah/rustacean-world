#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        })
    }

    pub fn read_winner(&self) -> (String, u16) {
        let (name1, score1) = &self.p1;
        let (name2, score2) = &self.p2;

        if score1 == score2 {
            ("Same score! tied".to_string(), *score1)
        } else if score1 > score2 {
            (name1.clone(), *score1)
        } else {
            (name2.clone(), *score2)
        }
    }

    pub fn update_score(&mut self, user_name: String) {
        let score1 = self.p1.1;
        let score2 = self.p2.1;
        let max_score = self.nb_games / 2 + 1;

        // Game finished?
        if score1 >= max_score || score2 >= max_score {
            return;
        }

        if self.p1.0 == user_name {
            self.p1.1 += 1;
        } else if self.p2.0 == user_name {
            self.p2.1 += 1;
        }
    }

    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}
