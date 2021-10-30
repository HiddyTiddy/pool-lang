use crate::pool::{tick, TickResponse};
use std::time::Instant;
use std::{io, sync::mpsc::channel, time::Duration};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::{Block, BorderType, Borders};

use crossterm::event::KeyCode;
use crossterm::{
    cursor,
    event::{self, Event as CEvent},
    execute, terminal,
};
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::widgets::Paragraph;
use tui::Terminal;

use crate::pool::{Grid, InterpretationState};

// ty ytop
fn setup_terminal() {
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();
    execute!(stdout, cursor::Hide).unwrap();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    terminal::enable_raw_mode().unwrap();
}

// ty ytop
fn cleanup_terminal() {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
    execute!(stdout, cursor::Show).unwrap();

    terminal::disable_raw_mode().unwrap();
}

enum Event<I> {
    Input(I),
    Tick,
}

fn render_grid<'a>(grid: &Grid, x: usize, y: usize) -> Vec<Spans<'a>> {
    let mut out = vec![];
    for yy in 0..grid.height {
        let mut line = vec![];
        let mut tmp = String::default();
        for xx in 0..grid.width {
            if x == xx && y == yy {
                line.push(Span::raw(tmp));
                tmp = String::default();
                line.push(Span::styled(
                    format!("{}", grid.grid[xx + yy * grid.width]),
                    Style::default().bg(Color::Red),
                ));
            } else {
                tmp.push(grid.grid[xx + yy * grid.width]);
            }
        }
        if !tmp.is_empty() {
            line.push(Span::raw(tmp));
        }
        out.push(Spans::from(line));
    }

    out
}

pub fn graphical_interpret(grid: Grid) -> Result<i64, Box<dyn std::error::Error>> {
    let (tx, rx) = channel();
    let tick_rate = Duration::from_millis(200);
    std::thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll failed") {
                if let CEvent::Key(key) = event::read().expect("failed to read events") {
                    tx.send(Event::Input(key)).expect("send failed");
                }
            }

            if last_tick.elapsed() >= tick_rate && tx.send(Event::Tick).is_ok() {
                last_tick = Instant::now();
            }
        }
    });

    setup_terminal();
    let backend = CrosstermBackend::new(io::stdout());
    let mut term = Terminal::new(backend).unwrap();

    // interpreter
    let mut interpretation_state = InterpretationState::new(grid.x0 as i64, grid.y0 as i64);
    let mut out_stream = String::default();
    let mut exited = false;

    loop {
        term.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(45),
                        Constraint::Percentage(40),
                        Constraint::Percentage(5),
                    ]
                    .as_ref(),
                )
                .split(size);

            let text = render_grid(
                &grid,
                interpretation_state.ptr.x as usize,
                interpretation_state.ptr.y as usize,
            );

            let program = Paragraph::new(text)
                .block(Block::default().title("Paragraph").borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Left)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default())
                        .border_type(BorderType::Plain),
                );
            // .wrap(Wrap { trim: true });

            rect.render_widget(program, chunks[1]);

            let text = vec![
                Spans::from(format!("Stack: {:?}", interpretation_state.stack)),
                Spans::from(format!("Heap: {:?}", &interpretation_state.heap[..128])),
                Spans::from(format!("Output: {:?}", &out_stream)),
            ];
            let program = Paragraph::new(text)
                .block(Block::default().title("Paragraph").borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Left)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default())
                        .border_type(BorderType::Plain),
                );
            // .wrap(Wrap { trim: true });

            rect.render_widget(program, chunks[2]);

            let footer = Paragraph::new("graphical pool interpreter")
                .style(Style::default().fg(Color::LightMagenta))
                .alignment(Alignment::Left)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default())
                        .border_type(BorderType::Plain),
                );
            rect.render_widget(footer, chunks[3]);
        })?;

        match rx.recv()? {
            Event::Input(key) => {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
            Event::Tick => {
                if !exited {
                    match tick(&grid, &mut interpretation_state) {
                        TickResponse::None => (),
                        TickResponse::Return(_) => exited = true,
                        TickResponse::Print(a) => {
                            // print!("{}", (a & 0xff) as u8 as char);
                            out_stream.push((a & 0xff) as u8 as char);
                        }
                    }
                }
            }
        }
    }

    cleanup_terminal();

    Ok(0)
}
