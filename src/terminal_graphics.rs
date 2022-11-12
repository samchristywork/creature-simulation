use crate::map;

use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{canvas::Canvas, Block, Borders},
    Terminal,
};

pub fn run<B: Backend>(terminal: &mut Terminal<B>, map: &map::Map) {
    let mut last_tick = Instant::now();
    loop {
        terminal
            .draw(|f| {
                let rect = Rect {
                    x: 0,
                    y: 0,
                    width: map.width as u16,
                    height: map.height as u16,
                };

                let canvas = Canvas::default()
                    .block(Block::default().borders(Borders::ALL).title("Map"))
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
                                            + map.slots[y as usize][x as usize]
                                                .encode_utf8(&mut tmp),
                                        Style::default().fg(color),
                                    ),
                                );
                            }
                        }
                    })
                    .x_bounds([rect.x as f64, rect.width as f64])
                    .y_bounds([rect.y as f64, rect.height as f64]);
                f.render_widget(canvas, rect);
            })
            .unwrap();

        let tick_rate = Duration::from_millis(250);
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if let KeyCode::Esc = key.code {
                    return;
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
