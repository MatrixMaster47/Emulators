use std::io::{stdout, Write};
use std::convert::TryInto;
use crossterm::{execute, queue, Result, 
    style::{Colorize, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, PrintStyledContent},
    event::{read, Event, KeyEvent, KeyCode},
    cursor::{position, MoveTo, Hide},
    terminal::{Clear, ClearType}};
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    None,
    Pawn(bool),
    Knight(bool),
    Bishop(bool),
    Rook(bool),
    Queen(bool),
    King(bool)
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum HighlightType {
     None,
    Inaccessible,
    Accessible,
    Movable,
    Winning
}

lazy_static! {
    static ref CMD_REGEX: Regex = Regex::new("(?P<srcX>[a-hA-H])(?P<srcY>[1-8]) to (?P<dstX>[a-hA-H])(?P<dstY>[1-8])").unwrap();
}



pub struct Game {
    pub board: [PieceType; 8*8],
    pub highlight_board: [HighlightType; 8*8],
    pub cmd: String,
    pub turn: bool,
    pub use_unicode: bool
}

impl Game {
    pub fn new(u: bool) -> Game {
        let mut g = Game {
            board: [PieceType::None; 64],
            highlight_board: [HighlightType::None; 64],
            cmd: String::new(),
            turn: false,
            use_unicode: u
        };
        g.reset_board();
        g
    }

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

        self.board[53] = PieceType::King(false);
    }

    pub fn draw_board(&self) -> Result<(u16,u16)> {
        let mut c: bool = false;
        let pos: (u16, u16) = position()?;
        
        /* While this clunky implementation is clunky, it's the simplest and
         * Probably actually the most performant*/
        queue!(stdout(),
            Print("# A B C D E F G H ")    
        )?;

        for i in 0..64 {
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

            queue!(stdout(), 
                SetBackgroundColor(match self.highlight_board[i] {
                    HighlightType::None         => if c { Color::DarkGrey }   else { Color::White },
                    HighlightType::Inaccessible => if c { Color::DarkRed }    else { Color::Red },
                    HighlightType::Accessible   => if c { Color::DarkYellow } else { Color::Yellow },
                    HighlightType::Movable      => if c { Color::DarkGreen }  else { Color::Green },
                    HighlightType::Winning      => if c { Color::DarkCyan }   else { Color::Cyan }
                }),
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
                let srci: usize = self.to_index(match split.name("srcX") { Some(some) => some.as_str(), None => {
                    self.cmd = String::new();
                    return ()
                }}, match split.name("dstX") { Some(some) => some.as_str().parse().unwrap(), None => {
                    self.cmd = String::new();
                    return ()
                }});
                 
                if(self.board[self.to_index(&split["srcX"], split["srcY"].parse().unwrap())] == 
                self.cmd = String::new();
            },
            _ => ()
        };
        let rg: Regex = Regex::new("(?P<srcX>[a-hA-H])(?P<srcY>[1-8])( to )?").unwrap();
        let capt = match rg.captures(&self.cmd) {
            None => {
                self.highlight_board = [HighlightType::None; 64];
                return
            },
            Some(some) => some
        };
       self.highlight(self.to_index(capt.name("srcX").unwrap().as_str(), capt.name("srcY").unwrap().as_str().parse().unwrap())); 
    }

    pub fn clear_board(&self, cursor: (u16, u16)) -> Result<()> {
        queue!(stdout(), MoveTo(cursor.0, cursor.1), Clear(ClearType::FromCursorDown))?;
        Ok(())
    }
    
    fn check_route(&self, src: usize, dst: Option<usize>) -> (bool, bool, bool) {
        match dst {
            None => {
                if self.board[src] == PieceType::None             
                || self.board[src] == PieceType::Pawn(!self.turn) 
                || self.board[src] == PieceType::Knight(!self.turn)
                || self.board[src] == PieceType::Rook(!self.turn) 
                || self.board[src] == PieceType::Bishop(!self.turn)
                || self.board[src] == PieceType::Queen(!self.turn)
                || self.board[src] == PieceType::King(!self.turn) {             
                    return (false, false, false)
                }
                return (true, false, false)
            },
            Some(s) => {
                let result: (bool, bool, bool) = (false, false, false);
                if self.board[src] == PieceType::None             
                || self.board[src] == PieceType::Pawn(!self.turn) 
                || self.board[src] == PieceType::Knight(!self.turn)
                || self.board[src] == PieceType::Rook(!self.turn) 
                || self.board[src] == PieceType::Bishop(!self.turn)
                || self.board[src] == PieceType::Queen(!self.turn)
                || self.board[src] == PieceType::King(!self.turn) {             
                    return result
                } else {
                    result.0 = true;

                    //TODO
                }
                
                return (false, false, false)
            }
        }; 
    }

    fn highlight(&mut self, i: usize) {
        if self.board[i] == PieceType::None             
        || self.board[i] == PieceType::Pawn(!self.turn) 
        || self.board[i] == PieceType::Knight(!self.turn)
        || self.board[i] == PieceType::Rook(!self.turn) 
        || self.board[i] == PieceType::Bishop(!self.turn)
        || self.board[i] == PieceType::Queen(!self.turn)
        || self.board[i] == PieceType::King(!self.turn) {
            self.highlight_board[i] = HighlightType::Inaccessible;
        } else {
            self.highlight_board[i] = HighlightType::Accessible;
        }
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

    fn from_index(&self, i: usize) -> (char, u8) {
        (match i-i/8*8 {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            4 => 'E',
            5 => 'F',
            6 => 'G',
            7 => 'H',
            _ => panic!("index is too large") 
        }, (8-i / 8).try_into().expect("index is too large!"))
    }
}
