use std::env;

mod chess;

fn main() -> Result<(), crossterm::ErrorKind> {
    let clear_pos: (u16, u16);
    let mut game = chess::Game {
        turn: false,
        use_unicode: if env::args().collect::<String>().contains("-u") { true } else { false },
        board: [chess::PieceType::None; 64],
        cmd: String::new()
    };

    game.reset_board();

    /* Draw the chess board before the game actually starts */
    clear_pos = game.draw_board()?;

    loop {
        match game.get_input() {
            Ok(Some(e)) => game.process(e),
            Ok(None) => continue,
            Err(err) => panic!("{}", err)
        };
        game.clear_board(clear_pos)?;
        game.draw_board()?;
    }
   Ok(())
}
