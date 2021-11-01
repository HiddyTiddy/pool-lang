use crate::pool::{tick, TickResponse};
use crate::util::PVec;
use std::convert::TryInto;
use std::time::Instant;
use std::{io, sync::mpsc::channel, time::Duration};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::Row;
use tui::widgets::Table;
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

fn render_grid<'a>(
    grid: &Grid,
    x: usize,
    y: usize,
    edit_cursor: PVec,
    show_cursor: bool,
) -> Vec<Spans<'a>> {
    let mut out = vec![];
    for yy in 0..grid.height {
        let mut line = vec![];
        let mut tmp = String::default();
        for xx in 0..grid.width {
            if show_cursor && edit_cursor.x as usize == xx && edit_cursor.y as usize == yy {
                line.push(Span::raw(tmp));
                tmp = String::default();
                line.push(Span::styled(
                    format!(
                        "{}",
                        grid[edit_cursor.x + edit_cursor.y * grid.width as i64]
                    ),
                    Style::default().bg(Color::LightCyan),
                ));
            } else if x == xx && y == yy {
                line.push(Span::raw(tmp));
                tmp = String::default();
                line.push(Span::styled(
                    format!("{}", grid.grid[xx + yy * grid.width]),
                    Style::default().bg(Color::Red),
                ));
            } else {
                let g = grid.grid[xx + yy * grid.width];
                tmp.push(if g == '\n' { ' ' } else { g });
            }
        }
        if !tmp.is_empty() {
            line.push(Span::raw(tmp));
        }
        out.push(Spans::from(line));
    }

    out
}

fn render_stack<'a>(stack: &[u64], is_focused: bool) -> Paragraph<'a> {
    let mut text = vec![];

    if !stack.is_empty() {
        text.push(Spans::from(format!("[[ {:5} ]]", stack[stack.len() - 1])));
        for i in (0..(stack.len() - 1)).rev() {
            text.push(Spans::from(format!(" [ {:5} ] ", stack[i])));
        }
    }

    Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("stack")
                .borders(Borders::ALL)
                .style(Style::default())
                // .border_type(BorderType::Plain),
                .border_type(if is_focused {
                    BorderType::Thick
                } else {
                    BorderType::Plain
                }),
        )
}

fn render_heap<'a>(heap: &[u64], width: u16, is_focused: bool) -> Table<'a> {
    let mut data = vec![];
    let mut line = vec![String::from("000000")];
    let mut count: u16 = 0;
    for (i, elem) in heap.iter().enumerate() {
        line.push(format!("{:02x}", elem));
        if (i) as u16 % (width / 5) == (width / 5) - 1 {
            data.push(Row::new(line));
            count += (width / 5) as u16;
            line = vec![format!("{:06x}", count)];
        }
    }
    if !line.is_empty() {
        data.push(Row::new(line));
    }
    Table::new(data)
        .widths(&[
            Constraint::Percentage(15),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
            Constraint::Percentage(5), // lol
        ])
        // .block(Block::default().title("Paragraph").borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .block(
            Block::default()
                .title("heap")
                .borders(Borders::ALL)
                .style(Style::default())
                .border_type(if is_focused {
                    BorderType::Thick
                } else {
                    BorderType::Plain
                }),
        )
}

fn render_output<'a>(output: &str, is_focused: bool) -> Paragraph<'a> {
    Paragraph::new(Spans::from(output.to_string()))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title("output")
                .borders(Borders::ALL)
                .style(Style::default())
                .border_type(if is_focused {
                    BorderType::Thick
                } else {
                    BorderType::Plain
                }),
        )
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
    let mut focus: u8 = 0;
    let mut paused = false;

    // editor
    let mut insert_mode = false;
    let mut edit_cursor = PVec {
        x: grid.x0 as i64,
        y: grid.y0 as i64,
    };

    loop {
        term.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let frame = Layout::default()
                .direction(Direction::Vertical)
                .vertical_margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);

            let text = render_grid(
                &grid,
                interpretation_state.ptr.x as usize,
                interpretation_state.ptr.y as usize,
                edit_cursor,
                paused,
            );

            let program = Paragraph::new(text)
                .block(Block::default().title("program").borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Left)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default())
                        .border_type(if focus == 0 {
                            BorderType::Thick
                        } else {
                            BorderType::Plain
                        }),
                );
            // .wrap(Wrap { trim: true });

            rect.render_widget(program, frame[0]);

            let info_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(33),
                        Constraint::Percentage(34),
                        Constraint::Percentage(33),
                    ]
                    .as_ref(),
                )
                .split(frame[1]);

            // let text = vec![
            //     Spans::from(format!("Stack: {:?}", interpretation_state.stack)),
            //     Spans::from(format!("Heap: {:?}", &interpretation_state.heap[..128])),
            //     Spans::from(format!("Output: {:?}", &out_stream)),
            // ];
            // let program = Paragraph::new(text)
            //     .block(Block::default().title("Paragraph").borders(Borders::ALL))
            //     .style(Style::default().fg(Color::White).bg(Color::Black))
            //     .alignment(Alignment::Left)
            //     .block(
            //         Block::default()
            //             .borders(Borders::ALL)
            //             .style(Style::default())
            //             .border_type(BorderType::Plain),
            //     );
            // // .wrap(Wrap { trim: true });

            // rect.render_widget(program, frame[1]);

            rect.render_widget(
                render_stack(&interpretation_state.stack, focus == 1),
                info_layout[0],
            );
            rect.render_widget(
                render_heap(&interpretation_state.heap, info_layout[1].width, focus == 2),
                info_layout[1],
            );
            rect.render_widget(render_output(&out_stream, focus == 3), info_layout[2]);

            let footer = Paragraph::new("graphical pool interpreter")
                .style(Style::default().fg(Color::LightMagenta))
                .alignment(Alignment::Left)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default())
                        .border_type(BorderType::Plain),
                );
            rect.render_widget(footer, chunks[2]);
        })?;

        match rx.recv()? {
            // todo insert and normal mode
            // i like vim
            Event::Input(key) => {
                if insert_mode {
                    cleanup_terminal();
                    todo!();
                } else {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Tab => focus = (focus + 1) % 4,
                        KeyCode::Char('r') => {
                            if exited {
                                exited = !exited;
                                interpretation_state =
                                    InterpretationState::new(grid.x0 as i64, grid.y0 as i64);
                                out_stream.clear();
                            }
                        }
                        KeyCode::Char(' ') => paused = !paused,
                        KeyCode::Char('.') => {
                            if !exited && paused {
                                match tick(&grid, &mut interpretation_state) {
                                    TickResponse::None => (),
                                    TickResponse::Return(_) => exited = true,
                                    TickResponse::Print(a) => {
                                        out_stream.push((a & 0xff) as u8 as char);
                                    }
                                    TickResponse::Panic(msg) => {
                                        cleanup_terminal();
                                        panic!("{}", msg);
                                    }
                                }
                            }
                        }
                        KeyCode::Char('?') => {
                            cleanup_terminal();
                            todo!("add help menu");
                        }
                        KeyCode::Down | KeyCode::Char('j') if edit_cursor.y +1< grid.height.try_into().unwrap() => {
                            edit_cursor += PVec { x: 0, y: 1 };
                        }
                        KeyCode::Up | KeyCode::Char('k') if edit_cursor.y > 0 => {
                            edit_cursor += PVec { x: 0, y: -1 };
                        }
                        KeyCode::Left | KeyCode::Char('h') if edit_cursor.x > 0 => {
                            edit_cursor += PVec { x: -1, y: 0 };
                        }
                        KeyCode::Right | KeyCode::Char('l') if edit_cursor.x +1< grid.width.try_into().unwrap() => {
                            edit_cursor += PVec { x: 1, y: 0 };
                        }
                        KeyCode::Char('i') => {
                            if focus == 0 && paused {
                                insert_mode = true;
                            }
                        }
                        _ => (),
                    }
                }
            }
            Event::Tick => {
                if !exited && !paused {
                    match tick(&grid, &mut interpretation_state) {
                        TickResponse::None => (),
                        TickResponse::Return(_) => exited = true,
                        TickResponse::Print(a) => {
                            // print!("{}", (a & 0xff) as u8 as char);
                            out_stream.push((a & 0xff) as u8 as char);
                        }
                        TickResponse::Panic(msg) => {
                            cleanup_terminal();
                            panic!("{}", msg);
                        }
                    }
                }
            }
        }
    }

    cleanup_terminal();

    Ok(0)
}
