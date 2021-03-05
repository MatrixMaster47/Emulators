use std::io::{stdout, Write};
use crossterm::{execute, queue, Result, ExecutableCommand, event, 
    style::{Colorize, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, PrintStyledContent},
    event::{read, Event, KeyEvent, KeyCode},
    cursor::{position, MoveTo, Hide},
    terminal::{Clear, ClearType}};
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Copy, Clone)]
pub enum PieceType {
    None,
    Pawn(bool),
    Knight(bool),
    Bishop(bool),
    Rook(bool),
    Queen(bool),
    King(bool)
}

lazy_static! {
    static ref CMD_REGEX: Regex = Regex::new("(?P<srcX>[a-hA-H])(?P<srcY>[1-8]) to (?P<dstX>[a-hA-H])(?P<dstY>[1-8])").unwrap();
}

pub struct Game {
    pub board: [PieceType; 64],
    pub cmd: String,
    pub turn: bool,
    pub use_unicode: bool
}

impl Game {
    /* Just hardcoded */
    pub fn reset_board(&mut self) {
        self.board[0] = PieceType::Rook(false);
        self.board[1] = PieceType::Knight(false);
        self.board[2] = PieceType::Bishop(false);
        self.board[3] = PieceType::Queen(false);
        self.board[4] = PieceType::King(false);
        self.board[5] = PieceType::Bishop(false);
        self.board[6] = PieceType::Knight(false);
        self.board[7] = PieceType::Rook(false);
        for i in 8..16 {
            self.board[i] = PieceType::Pawn(false);
        }

        for i in 17..47 {
            self.board[i] = PieceType::None;
        }

        self.board[56] = PieceType::Rook(true);
        self.board[57] = PieceType::Knight(true);
        self.board[58] = PieceType::Bishop(true);
        self.board[59] = PieceType::Queen(true);
        self.board[60] = PieceType::King(true);
        self.board[61] = PieceType::Bishop(true);
        self.board[62] = PieceType::Knight(true);
        self.board[63] = PieceType::Rook(true);
        for i in 48..56 {
            self.board[i] = PieceType::Pawn(true);
        }

    }

    pub fn draw_board(&self) -> Result<(u16,u16)> {
        let mut c: bool = false;
        let pos: (u16, u16) = position()?;

        queue!(stdout(),
            Print("# A B C D E F G H ")    
        )?;

        for i in 0..64 {
            /*if i < 8 {
                queue!(stdout(),
                    Print(format!("{}", match i {
                        0 => 'A',
                        1 => 'B',
                        2 => 'C',
                        3 => 'D',
                        4 => 'E',
                        5 => 'F',
                        6 => 'G',
                        7 => 'H',
                        _ => panic!("ERROR")
                    }))
                )?;
            }*/
            /* If i is dividable by 8 then we're at the end of a line */
            if i % 8 == 0 {
                queue!(stdout(), 
                    SetBackgroundColor(Color::Reset),
                    Print(format!("\n{} ", 8-(i/8)))
                )?;
                /* Invert c so the next cell (first of next line) 
                 * is the same as the current line like
                 * BWBW -> ends with W
                 * WBWB -> also starts with W 
                 * ... */
                c = !c; 
            }
            queue!(stdout(), SetBackgroundColor(if c { Color::White } else { Color::DarkGrey }),
                PrintStyledContent(match self.use_unicode { 
                    true => match self.board[i] {
                        PieceType::None          => "  ".white(),
                        PieceType::Pawn(false)   => "♟︎".black(),
                        PieceType::Knight(false) => "♞".black(),
                        PieceType::Bishop(false) => "♝".black(),
                        PieceType::Rook(false)   => "♜".black(),
                        PieceType::Queen(false)  => "♛".black(),
                        PieceType::King(false)   => "♚".black(),
                        PieceType::Pawn(true)    => "♟︎".grey(),
                        PieceType::Knight(true)  => "♞".grey(),
                        PieceType::Bishop(true)  => "♝".grey(),
                        PieceType::Rook(true)    => "♜".grey(),
                        PieceType::Queen(true)   => "♛".grey(),
                        PieceType::King(true)    => "♚".grey()
                    },
                    false => match self.board[i] {
                        PieceType::None          => "  ".white(),
                        PieceType::Pawn(false)   => "P ".black(),
                        PieceType::Knight(false) => "H ".black(),
                        PieceType::Bishop(false) => "B ".black(),
                        PieceType::Rook(false)   => "R ".black(),
                        PieceType::Queen(false)  => "Q ".black(),
                        PieceType::King(false)   => "K ".black(),
                        PieceType::Pawn(true)    => "P ".grey(),
                        PieceType::Knight(true)  => "H ".grey(),
                        PieceType::Bishop(true)  => "B ".grey(),
                        PieceType::Rook(true)    => "R ".grey(),
                        PieceType::Queen(true)   => "Q ".grey(),
                        PieceType::King(true)    => "K ".grey()
                    }
                }))?;
            c = !c;
        }
        queue!(stdout(), SetBackgroundColor(Color::Reset), Print(format!("\n\n> {}", self.cmd)))?;
        stdout().flush()?;
        //Ok(position()?)
        Ok(pos)
    }
    
    pub fn get_input(&self) -> Result<Option<KeyEvent>> {
        match read()? {
            Event::Key(e)      => Ok(Some(e)),
            Event::Mouse(_)    => Ok(None),
            Event::Resize(_,_) => Ok(None)
        }
    }

    pub fn process(&mut self, e: KeyEvent) {
        match e.code {
            KeyCode::Char(c)   => self.cmd.push(c),
            KeyCode::Backspace => drop(self.cmd.pop()),
            KeyCode::Enter     => {
                if self.cmd == "new" || self.cmd == "reset" {
                    self.reset_board();
                }

                let split = match CMD_REGEX.captures(&self.cmd) {
                    Some(some) => some,
                    None => {
                        self.cmd = String::new();
                        return ()
                    }   
                }; 
                self.board[self.to_index(&split["srcX"], split["srcY"].parse().unwrap())] = PieceType::King(true); 
                self.cmd = String::new();
            },
            _ => ()
        };
    }

    pub fn clear_board(&self, cursor: (u16, u16)) -> Result<()> {
        queue!(stdout(), MoveTo(cursor.0, cursor.1), Clear(ClearType::FromCursorDown))?;
        stdout().flush()?;
        Ok(())
    }

    fn to_index(&self, c: &str, n: u8) -> usize {
        (match c {
            "a"|"A" => 0,
            "b"|"B" => 1,
            "c"|"C" => 2,
            "d"|"D" => 3,
            "e"|"E" => 4,
            "f"|"F" => 5,
            "g"|"G" => 6,
            "h"|"H" => 7,
            _       => 0
        } + (8-n) * 8) as usize
    }
}
