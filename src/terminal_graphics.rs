use crate::map;
use crate::position::Position;
use crate::world;
use crossterm::event::{self, Event, KeyCode};
use std::collections::HashMap;
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

#[derive(PartialEq, Eq)]
pub enum Interaction {
    Back,
    Down,
    Forward,
    Halt,
    Left,
    Pause,
    Progress,
    Right,
    SlowDown,
    SpeedUp,
    ToggleShowDead,
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
    frame_delay: u64,
    cursor: &Cursor,
    world_state: &world::WorldState,
    show_dead: bool,
) -> Interaction {
    terminal
        .draw(|f| {
            let rect = Rect {
                x: 2,
                y: 2,
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
                        4.0,
                        2.0,
                        Span::styled(
                            format!(
                                "{} ({}) {}",
                                frame_count,
                                frame_delay,
                                world_state.num_alive()
                            ),
                            Style::default().fg(Color::Magenta),
                        ),
                    );
                    if cursor.show {
                        ctx.print(
                            f64::from(cursor.x),
                            f64::from(cursor.y),
                            Span::styled(
                                format!(
                                    "{}",
                                    world_state
                                        .get_creatures_at(Position::new(cursor.x, cursor.y))
                                        .len()
                                ),
                                Style::default().fg(Color::Yellow),
                            ),
                        );
                    }
                })
                .x_bounds([f64::from(rect.x), f64::from(rect.width)])
                .y_bounds([f64::from(rect.y), f64::from(rect.height)]);
            let mut size = f.size();
            size.width = std::cmp::min(map.width as u16, size.width);
            size.height = std::cmp::min(map.height as u16, size.height);
            f.render_widget(canvas, size);

            let mut info_box_text = Vec::new();
            for creature in world_state.get_creatures_at(Position::new(cursor.x, cursor.y)) {
                if creature.is_alive() || show_dead {
                    info_box_text.push(Spans::from(format!("{}", creature)))
                }
            }
            let info_box = Paragraph::new(info_box_text)
                .block(Block::default().title("Info").borders(Borders::ALL))
                .wrap(Wrap { trim: true });

            let mut histogram: HashMap<u64, u64> = HashMap::new();
            for creature in &world_state.creatures {
                if creature.is_alive() {
                    if histogram.get(&creature.strain).is_none() {
                        histogram.insert(creature.strain, 0);
                    }
                    let count = histogram.get(&creature.strain);
                    histogram.insert(
                        creature.strain,
                        count.expect("Could not get strain count.") + 1,
                    );
                }
            }
            let mut leaderboard_values = Vec::new();
            for element in histogram {
                leaderboard_values.push(element)
            }
            leaderboard_values.sort_by(|a, b| {
                if a.1 == b.1 {
                    b.0.partial_cmp(&a.0)
                        .expect("Could not perform comparison.")
                } else {
                    b.1.partial_cmp(&a.1)
                        .expect("Could not perform comparison.")
                }
            });
            let mut leaderboard_text = Vec::new();
            for element in leaderboard_values {
                leaderboard_text.push(Spans::from(format!("{} {}", element.0, element.1)))
            }
            let leaderboard = Paragraph::new(leaderboard_text)
                .block(Block::default().title("Leaderboard").borders(Borders::ALL))
                .wrap(Wrap { trim: true });

            if f.size().height - size.height > 10 {
                size.y = size.height;
                size.height = 10;
                size.x = 0;
                size.width /= 2;
                f.render_widget(info_box, size);
                size.x = size.width;
                f.render_widget(leaderboard, size);
            }
        })
        .expect("Could not perform draw.");

    if crossterm::event::poll(Duration::from_millis(frame_delay)).expect("Could not poll events.") {
        if let Event::Key(key) = event::read().expect("Could not read key signature.") {
            match key.code {
                KeyCode::Char(' ') => return Interaction::Pause,
                KeyCode::Char(',') => return Interaction::Back,
                KeyCode::Char('.') => return Interaction::Forward,
                KeyCode::Char('d') => return Interaction::ToggleShowDead,
                KeyCode::Char('h') => return Interaction::Left,
                KeyCode::Char('j') => return Interaction::Down,
                KeyCode::Char('k') => return Interaction::Up,
                KeyCode::Char('l') => return Interaction::Right,
                KeyCode::Char('p') => return Interaction::Pause,
                KeyCode::Char('q') => return Interaction::Halt,
                KeyCode::Char('[') => return Interaction::SpeedUp,
                KeyCode::Char(']') => return Interaction::SlowDown,
                KeyCode::Down => return Interaction::Down,
                KeyCode::Esc => return Interaction::Halt,
                KeyCode::Left => return Interaction::Left,
                KeyCode::Right => return Interaction::Right,
                KeyCode::Up => return Interaction::Up,
                key => println!("Not handled: {:?}", key),
            }
        }
    }
    Interaction::Progress
}
