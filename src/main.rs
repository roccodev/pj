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

extern crate crossterm;
extern crate serde_json;
extern crate tui;

pub mod json;

use crossterm::{input, Screen, ClearType, Crossterm, TerminalInput};

use std::io::{self, stdout, Read, Write};

use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Corner, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Paragraph, SelectableList, Text, Widget};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let mut json = String::new();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    stdin
        .read_to_string(&mut json)
        .expect("Could not read from STDIN");

    let elements = json::parse(json).expect("Could not parse JSON.");

    let screen = Screen::default();
    let alt = screen.enable_alternate_modes(true).unwrap();
    let backend = CrosstermBackend::with_alternate_screen(alt).unwrap();

    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();


    let elements_iter = elements.iter();

    let mut keys: Vec<String> = vec![];

    for (ref key, ref _value) in elements_iter {
        &keys.push(key.to_string());
    }

    let mut selected = None;

    let mut i_key = crossterm::input().read_async().bytes();

    let mut arrow_stage = 0;

    loop {
        let key_r = i_key.next();
        if key_r.is_some() {
            let key_opt = key_r.unwrap();
            if key_opt.is_ok() {
                let key = key_opt.unwrap();

                match key {
                    113 /* 'q' */ => {
                        return Ok(());
                    }

                    /* Arrow stages

                        Arrow inputs are sent one after each other, in this order:
                        27 91 [key]

                        where [key] is 65 for Up, 66 for Down and so on.
                    */

                    27 => {
                        if arrow_stage == 0 {
                            arrow_stage = 1;
                        }
                    }

                    91 =>  {
                        if arrow_stage == 1 {
                            arrow_stage = 2;
                        }
                    }

                    /* Up */
                    65 => {
                        if arrow_stage == 2 {
                            arrow_stage = 0;

                            selected = 
                            if let Some(selected) = selected {
                                if selected > 0 {
                                    Some(selected - 1)
                                } else {
                                    Some(keys.len() - 1)
                                }
                            } else {
                                Some(0)
                            }
                        }
                    }

                    /* Down */
                    66 => {
                        if arrow_stage == 2 {
                            arrow_stage = 0;

                            selected = 
                            if let Some(selected) = selected {
                                if selected >= keys.len() - 1 {
                                    Some(0)
                                } else {
                                    Some(selected + 1)
                                }
                            } else {
                                Some(0)
                            }
                        }
                    }

                    /* End */
                    70 => {
                        if arrow_stage == 2 {
                            arrow_stage = 0;

                            selected = Some(elements.len() - 1);
                        }
                    }

                    /* Home */
                    72 => {
                        if arrow_stage == 2 {
                            arrow_stage = 0;

                            selected = Some(0);
                        }
                    }
                    _ => {}
                }
            }
        }

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
    }
}
