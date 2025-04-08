pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    let mut player_x = 0;
    let mut player_o = 0;
    let mut empty = 0;

    for i in 0..3 {
        for j in 0..3 {
            match table[i][j] {
                'X' => player_x += 1,
                'O' => player_o += 1,
                '#' => empty += 1,
                _ => {}
            }
        }
    }

    if player_x > player_o + empty {
        return "tie".to_string();
    } else if player_o > player_x + empty {
        return "tie".to_string();
    }

    if horizontal('X', table) || vertical('X', table) || diagonals('X', table) {
        return "player X won".to_string();
    } else if horizontal('O', table) || vertical('O', table) || diagonals('O', table) {
        return "O".to_string();
    } else if empty == 0 {
        return "tie".to_string();
    } else {
        return "player 0 won".to_string();
    }
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    // Check main diagonal (top-left to bottom-right)
    let main_diagonal = table[0][0] == player && table[1][1] == player && table[2][2] == player;
    
    // Check secondary diagonal (top-right to bottom-left)
    let secondary_diagonal = table[0][2] == player && table[1][1] == player && table[2][0] == player;
    
    main_diagonal || secondary_diagonal
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    // Check each row
    for row in table.iter() {
        if row[0] == player && row[1] == player && row[2] == player {
            return true;
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    // Check each column
    for col in 0..3 {
        if table[0][col] == player && table[1][col] == player && table[2][col] == player {
            return true;
        }
    }
    false
}
