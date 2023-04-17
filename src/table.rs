use std::io::{Stdout, stdout, Write};
use crossterm::{QueueableCommand,
                terminal::{self, SetSize, enable_raw_mode, disable_raw_mode, Clear, ClearType},
                cursor::{self, MoveTo},
                style::{Stylize, Color, PrintStyledContent, Attribute, Print, SetAttribute, SetBackgroundColor},
                event::{read, poll, Event, KeyCode, KeyEventKind},
                Result};
use crossterm::style::Color::{Black, White};
use crossterm::style::{SetColors, SetForegroundColor};

struct Character {
    character: char,
    fg_color: Color,
    bg_color: Color
}

struct Change {
    column: usize,
    row: usize,
    symbol: Character
}

struct View {
    screen_map: Vec<Vec<Character>>,
    screen_prev_map: Vec<Vec<Character>>,
    screen_id: usize,

    changes: Vec<Change>,

    stdout: Stdout
}

impl View {
    fn new_view(width: usize, height: usize, id: usize) -> Self {
        View {
            screen_map: vec![vec![Character{
                character: ' ',
                fg_color: White,
                bg_color: Black
            };width];height],
            screen_prev_map: vec![vec![Character{
                character: ' ',
                fg_color: White,
                bg_color: Black
            };width];height],
            screen_id: id,
            changes: vec![],
            stdout: stdout()
        }
    }

    fn redraw(&mut self) -> Result<()> {
        let mut current_colors = (Black, White);
        for (column, line) in self.screen_map.iter().enumerate() {
            for (row, symbol) in line.iter().enumerate() {
                self.stdout.queue(MoveTo(column as u16, row as u16))?;
                if current_colors != (symbol.fg_color, symbol.bg_color) {
                    current_colors = (symbol.fg_color, symbol.bg_color);
                    self.stdout.queue(SetForegroundColor(symbol.fg_color))?;
                    self.stdout.queue(SetBackgroundColor(symbol.bg_color))?;
                }
                self.stdout.queue(Print(symbol.character))?;
            }
        }
        self.stdout.flush()?;
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        let mut current_colors = (Black, White);
        for change in self.changes.iter() {
            self.stdout.queue(MoveTo(column as u16, row as u16))?;
            if current_colors != (change.symbol.fg_color, change.symbol.bg_color) {
                current_colors = (change.symbol.fg_color, change.symbol.bg_color);
                self.stdout.queue(SetForegroundColor(change.symbol.fg_color))?;
                self.stdout.queue(SetBackgroundColor(change.symbol.bg_color))?;
            }
            self.stdout.queue(Print(symbol.character))?;
        }
        self.stdout.flush()?;
        self.changes.clear();
        Ok(())
    }
}