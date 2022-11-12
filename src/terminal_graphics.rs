use crate::map;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{canvas::Canvas, Block, Borders},
    Terminal,
};

fn run<B: Backend>(terminal: &mut Terminal<B>, map: &map::Map) {
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
                        ctx.print(
                            10.0,
                            10.0,
                            Span::styled("You are here", Style::default().fg(Color::Yellow)),
                        );
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

pub fn display(map: &map::Map) {
    // Init
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // Run
    run(&mut terminal, map);

    // Cleanup
    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    terminal.show_cursor().unwrap();
}
