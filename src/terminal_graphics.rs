use crate::map;
use crate::position::Position;
use crate::world;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use tui::{
    backend::Backend,
    text::Spans,
    widgets::{Paragraph, Wrap},
};
use tui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{canvas::Canvas, Block, Borders},
    Terminal,
};

#[derive(PartialEq)]
pub enum Interaction {
    Back,
    Down,
    Forward,
    Halt,
    Left,
    Pause,
    Progress,
    Right,
    Up,
}

pub struct Cursor {
    pub show: bool,
    pub x: i32,
    pub y: i32,
}

pub fn display<B: Backend>(
    terminal: &mut Terminal<B>,
    map: &map::Map,
    frame_count: usize,
    frame_delay: Duration,
    cursor: &Cursor,
    world_state: &world::WorldState,
) -> Interaction {
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
                            let shade = map.slots[y as usize][x as usize].1 as u8;
                            let color = match map.slots[y as usize][x as usize].0 {
                                '.' => Color::Green,
                                ' ' => Color::Reset,
                                'x' => Color::Rgb(30, 30, 30),
                                _ => Color::Rgb(shade, shade, shade),
                            };
                            ctx.print(
                                x as f64,
                                y as f64,
                                Span::styled(
                                    String::new()
                                        + map.slots[y as usize][x as usize].0.encode_utf8(&mut tmp),
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
                    if cursor.show {
                        ctx.print(
                            cursor.x as f64,
                            cursor.y as f64,
                            Span::styled(format!("X"), Style::default().fg(Color::Yellow)),
                        );
                    }
                })
                .x_bounds([rect.x as f64, rect.width as f64])
                .y_bounds([rect.y as f64, rect.height as f64]);
            let mut size = f.size();
            size.width = std::cmp::min(map.width as u16, size.width);
            size.height = std::cmp::min(map.height as u16, size.height);
            f.render_widget(canvas, size);

            let mut text = Vec::new();
            for creature in world_state.get_creatures_at(Position::new(cursor.x, cursor.y)) {
                text.push(Spans::from(format!("{}", creature)))
            }
            let p = Paragraph::new(text)
                .block(Block::default().title("Info").borders(Borders::ALL))
                .wrap(Wrap { trim: true });

            if f.size().height - size.height > 10 {
                size.y = size.height;
                size.height = 10;
                size.x = 0;
                f.render_widget(p, size);
            }
        })
        .unwrap();

    if crossterm::event::poll(frame_delay).unwrap() {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char(' ') => return Interaction::Pause,
                KeyCode::Char(',') => return Interaction::Back,
                KeyCode::Char('.') => return Interaction::Forward,
                KeyCode::Char('h') => return Interaction::Left,
                KeyCode::Char('j') => return Interaction::Down,
                KeyCode::Char('k') => return Interaction::Up,
                KeyCode::Char('l') => return Interaction::Right,
                KeyCode::Char('p') => return Interaction::Pause,
                KeyCode::Char('q') => return Interaction::Halt,
                KeyCode::Down => return Interaction::Down,
                KeyCode::Esc => return Interaction::Halt,
                KeyCode::Left => return Interaction::Left,
                KeyCode::Right => return Interaction::Right,
                KeyCode::Up => return Interaction::Up,
                key => println!("Not handled: {:?}", key),
            }
        }
    }
    return Interaction::Progress;
}
