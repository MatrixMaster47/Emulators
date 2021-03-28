use std::env;

mod chess;

fn main() -> Result<(), crossterm::ErrorKind> {
    let mut game = chess::Game::new(if env::args().collect::<String>().contains("-u") { true } else { false });

    /* Draw the chess board before the game actually starts */
    let clear_pos = game.draw_board()?;

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
