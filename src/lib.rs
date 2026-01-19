#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_starts_with_all_cells_untouched() {
        let game = Game::new();
        let board = game.player_board();

        for cell in board.cells() {
            assert_eq!(cell.state(), CellState::Untouched);
        }
    }
}
