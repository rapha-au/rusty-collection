#![warn(clippy::all, clippy::pedantic)]
// SNAKE GAME IN RUST
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use std::io::{self, stdout, Read, Stdout, Write};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, SetSize, SetTitle},
    Command,
    ExecutableCommand,
};

use rand::random;
use rand::thread_rng;
use rand::Rng;


pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    hook: Stdout,
}


impl Terminal {
    pub fn default() -> Self {
        enable_raw_mode();
        Self {
            size: Size {
                width: 75,
                height: 35,
            },
            hook: stdout(),
        }
    }

    pub fn quit(&mut self) {
        panic!();
    }

    pub fn enable_raw(&mut self) {
        enable_raw_mode();
    }

    pub fn set_title(&mut self, title: &str) {
        execute!(self.hook, SetTitle(title));
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }

    pub fn set_size(&mut self, cols: u16, rows: u16) {
        execute!(self.hook, SetSize(cols, rows));
        self.size = Size {
            width: cols,
            height: rows,
        };
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        self.hook.flush()
    }

    pub fn clear_screen(&mut self) {
        execute!(self.hook, Clear(ClearType::All));
    }

    pub fn cursor_hide(&mut self) {
        execute!(self.hook, cursor::Hide);
    }

    pub fn move_cursor(&mut self, x: u16, y: u16) {
        execute!(self.hook, cursor::MoveTo(x, y));
    }

    pub fn put_glyph(&mut self, chr: char, x: u16, y: u16, fg: Color, bg: Color) {
        self.move_cursor(x, y);

        execute!(
            self.hook,
            SetForegroundColor(fg),
            SetBackgroundColor(bg),
            Print(chr),
            ResetColor
        );
    }

    pub fn put_str(&mut self, line_str: String, x: u16, y: u16, fg: Color, bg: Color) {
        self.move_cursor(x, y);
        //println!("{}\r",line_str.to_string());

        execute!(
            self.hook,
            SetForegroundColor(fg),
            SetBackgroundColor(bg),
            Print(line_str.to_string()),
            ResetColor
        );
    }

    pub fn draw_rect(&mut self, x: u16, y: u16, w: u16, h: u16, border_color: Color) {
        let h_line = ('\u{2501}').to_string().repeat(w as usize);

        let v_line = "\u{2503}"; //2502

        //Horizontal Lines
        self.put_str(h_line.to_string(), x + 1, y, border_color, Color::Reset);
        self.put_str(h_line.to_string(), x + 1, y + h, border_color, Color::Reset);

        //Vertical Lines
        for i in y + 1..y + h {
            self.put_str(v_line.to_string(), x, i, border_color, Color::Reset);
            self.put_str(v_line.to_string(), x + w, i, border_color, Color::Reset);
        }

        //Down and right
        self.put_str("\u{250F}".to_string(), x, y, border_color, Color::Reset);
        //Down and left
        self.put_str("\u{2513}".to_string(), x + w, y, border_color, Color::Reset);
        //Up Right
        self.put_str("\u{2517}".to_string(), x, y + h, border_color, Color::Reset);
        //Up Left
        self.put_str(
            "\u{251B}".to_string(),
            x + w,
            y + h,
            border_color,
            Color::Reset,
        );
    }
}

#[derive(PartialEq)]
enum direction {
    up,
    down,
    left,
    right,
}

#[derive(Clone)]
pub struct Position {
    x: u16,
    y: u16,
}


pub struct Food {
    glyph: char,
    position: Position,
    active: bool,
}

impl Food {
    pub fn default() -> Self {
        Self {
            glyph: '*',
            position: Position {
                x: 3,
                y: 3,
            },
            active: true,
        }
    }

    pub fn set_pos(&mut self) {}
}

pub struct Snake {
    glyph: char,
    body: VecDeque<Position>,
    move_dir: direction,
	food_count:u128
}

impl Snake {
    pub fn default() -> Self {
        //3 size initially
        Self {
            glyph: '#',
            body: VecDeque::from(vec![
                Position { x: 5, y: 5 },
                Position { x: 5, y: 6 },
                Position { x: 5, y: 7 },
            ]),
            move_dir: direction::down,
			food_count:0
        }
		
    }
	
	pub fn get_foodcount(&self)->u128{
		self.food_count
	}

    pub fn increase_segment(&mut self) {
        match self.move_dir {
            direction::up => {
                self.body.push_back(Position {
                    x: self.body.iter().last().unwrap().x,
                    y: self.body.iter().last().unwrap().y - 1,
                });
            }
            direction::down => {
                self.body.push_back(Position {
                    x: self.body.iter().last().unwrap().x,
                    y: self.body.iter().last().unwrap().y + 1,
                });
            }
            direction::left => {
                self.body.push_back(Position {
                    x: self.body.iter().last().unwrap().x - 1,
                    y: self.body.iter().last().unwrap().y,
                });
            }
            direction::right => {
                self.body.push_back(Position {
                    x: self.body.iter().last().unwrap().x + 1,
                    y: self.body.iter().last().unwrap().y,
                });
            }
            _ => {}
        }
		self.food_count+=1;
    }

    pub fn process_keypress(&mut self, term: &mut Terminal) {
        if let Event::Key(key) = read().unwrap() {
            match key {
                // CONTROL MODIFIERS

                //Quit
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    term.quit();
                }

                //MOVEMENT
                KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    if self.move_dir != direction::down {
                        self.move_dir = direction::up;
                    }
                }
                KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    if self.move_dir != direction::up {
                        self.move_dir = direction::down;
                    }
                }
                KeyEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    if self.move_dir != direction::right {
                        self.move_dir = direction::left;
                    }
                }
                KeyEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    if self.move_dir != direction::left {
                        self.move_dir = direction::right;
                    }
                }

                _ => (),
            }
        };
    }

    pub fn movement_update(&mut self) {
        match self.move_dir {
            direction::up => {
                self.body.push_back(Position {
                    x: self.body.iter().last().unwrap().x,
                    y: self.body.iter().last().unwrap().y - 1,
                });
            }
            direction::down => {
                self.body.push_back(Position {
                    x: self.body.iter().last().unwrap().x,
                    y: self.body.iter().last().unwrap().y + 1,
                });
            }
            direction::left => {
                self.body.push_back(Position {
                    x: self.body.iter().last().unwrap().x - 1,
                    y: self.body.iter().last().unwrap().y,
                });
            }
            direction::right => {
                self.body.push_back(Position {
                    x: self.body.iter().last().unwrap().x + 1,
                    y: self.body.iter().last().unwrap().y,
                });
            }
            _ => {}
        }

        //Remove back part of the tail by popping front
        self.body.pop_front();
    }

    pub fn get_body(&self) -> VecDeque<Position> {
        self.body.clone()
    }
}

fn main() {
	
    let mut term = Terminal::default();
    term.set_size(term.get_size().width, term.get_size().height);
    term.set_title("Snake");

    term.enable_raw();
    term.cursor_hide();

    let now = Instant::now();

    let mut snake = Snake::default();

    let mut food = Food::default();

    loop {
		
		term.put_str(snake.get_foodcount().to_string(),0,0, Color::White, Color::Black);
		
		term.put_str("Press 'q' to quit".to_string(),0,term.get_size().height-1, Color::White, Color::Black);
		
        if (now.elapsed().as_millis() % 500 == 0) {
            if poll(Duration::from_millis(0)).unwrap() {
                snake.process_keypress(&mut term);
            }

            snake.movement_update();

            term.clear_screen();
			
			//Draw Area
			term.draw_rect(
			0, 0,
			term.get_size().width-1, term.get_size().height-1,
			Color::Red);
			

            let mut s_body = snake.get_body();

            //Draw Snake
            for segment in s_body.iter() {
                term.put_glyph('#', segment.x, segment.y, Color::Green, Color::Black);
            }

            for segment in s_body.iter() {
                // Check Snake/Food Collision
                if segment.x == food.position.x && segment.y == food.position.y {
                    snake.increase_segment();
                    food.position.x = thread_rng().gen_range(5..term.get_size().width - 5);
                    food.position.y = thread_rng().gen_range(5..term.get_size().height - 5);
                }

                //Check Snake Border Collision
                if segment.x == 0
                    || segment.y == 0
                    || segment.x == term.get_size().width - 1
                    || segment.y == term.get_size().height - 1
                {
                    panic!();
                }
            }

            term.put_glyph(
                food.glyph,
                food.position.x,
                food.position.y,
                Color::Yellow,
                Color::Black,
            );
        }

        term.flush();
    }
}
