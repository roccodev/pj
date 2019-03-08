// Copyright (C) 2019 RoccoDev
//
// This file is part of pj.
//
// pj is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// pj is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with pj.  If not, see <http://www.gnu.org/licenses/>.

extern crate serde_json;
extern crate termion;
extern crate tui;

pub mod events;
pub mod json;

use std::io::{self, Read};
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Alignment, Constraint, Corner, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Paragraph, SelectableList, Text, Widget};
use tui::Terminal;

use events::{Event, Events};

fn main() -> Result<(), io::Error> {
    let mut json = String::new();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    stdin
        .read_to_string(&mut json)
        .expect("Could not read from STDIN");

    let tty = termion::get_tty().unwrap();

    let stdout = tty.into_raw_mode().expect("Failed to get STDOUT");
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();

    let elements = json::parse(json).unwrap();

    let elements_iter = elements.iter();

    let mut keys: Vec<String> = vec![];

    for (ref key, ref value) in elements_iter {
        &keys.push(key.to_string());
    }

    let evts = Events::new();

    let mut selected = None;

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let style = Style::default().fg(Color::Black).bg(Color::White);
            SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title("JSON Keys"))
                .items(&keys)
                .select(selected)
                .style(style)
                .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::Bold))
                .highlight_symbol(">")
                .render(&mut f, chunks[0]);
            {
                let (ref _key, ref value) = elements[selected.unwrap_or(0)];
                let text = [Text::raw(value)];

                Paragraph::new(text.iter())
                    .block(Block::default().borders(Borders::ALL).title("JSON Values"))
                    .alignment(Alignment::Left)
                    .render(&mut f, chunks[1]);
            }
        })?;

        match evts.next().unwrap() {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    return Ok(());
                }
                Key::Left => {
                    selected = None;
                }
                Key::Down => {
                    selected = if let Some(selected) = selected {
                        if selected >= keys.len() - 1 {
                            Some(0)
                        } else {
                            Some(selected + 1)
                        }
                    } else {
                        Some(0)
                    }
                }
                Key::Up => {
                    selected = if let Some(selected) = selected {
                        if selected > 0 {
                            Some(selected - 1)
                        } else {
                            Some(keys.len() - 1)
                        }
                    } else {
                        Some(0)
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
