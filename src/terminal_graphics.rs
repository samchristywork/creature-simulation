use crate::map;

use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{canvas::Canvas, Block, Borders},
    Terminal,
};

#[derive(PartialEq)]
pub enum Continuation {
    Halt,
    Progress,
}

pub fn display<B: Backend>(
    terminal: &mut Terminal<B>,
    map: &map::Map,
    frame_count: i32,
) -> Continuation {
    terminal
        .draw(|f| {
            let rect = Rect {
                x: 0,
                y: 0,
                width: map.width as u16,
                height: map.height as u16,
            };

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(map.name.as_str()),
                )
                .paint(|ctx| {
                    for x in 0..map.width {
                        for y in 0..map.height {
                            let mut tmp = [0];
                            let color = match map.slots[y as usize][x as usize] {
                                'c' => Color::Yellow,
                                '.' => Color::Green,
                                _ => Color::Reset,
                            };
                            ctx.print(
                                x as f64,
                                y as f64,
                                Span::styled(
                                    String::new()
                                        + map.slots[y as usize][x as usize].encode_utf8(&mut tmp),
                                    Style::default().fg(color),
                                ),
                            );
                        }
                    }
                    ctx.print(
                        0 as f64,
                        0 as f64,
                        Span::styled(
                            format!("{}", frame_count),
                            Style::default().fg(Color::Magenta),
                        ),
                    );
                })
                .x_bounds([rect.x as f64, rect.width as f64])
                .y_bounds([rect.y as f64, rect.height as f64]);
            f.render_widget(canvas, rect);
        })
        .unwrap();

    if crossterm::event::poll(Duration::from_millis(100)).unwrap() {
        if let Event::Key(key) = event::read().unwrap() {
            if let KeyCode::Esc = key.code {
                return Continuation::Halt;
            }
        }
    }
    return Continuation::Progress;
}
