use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyModifiers},
    execute,
    style::Color,
    style::{Stylize, self},
    terminal,
    terminal::size,
    ExecutableCommand, Result,
};
use rand::prelude::*;

use std::io::{stdout, Write};
use std::time::Duration;

use crate::{charset::get_random_char, cmdline::CmdOptions};
use unicode_width::UnicodeWidthChar;

const EFFECT_SPEED: i32 = 4;
const JUMBLE_SECS: i32 = 2;
const JUMBLE_LOOP_SPEED: i32 = 35;
const REVEAL_LOOP_SPEED: i32 = 150;

#[derive(Debug)]
struct CharAttr {
    src: char,
    mask: char,
    width: usize,
    is_space: bool,
    time: Duration,
}

#[derive(Debug)]
pub(crate) struct EffectLauncher<'a> {
    opts: &'a CmdOptions
}

fn wait_for_input() -> Result<()> {
    terminal::enable_raw_mode()?;

    loop {
        if let Event::Key(e) = read()? {
            terminal::disable_raw_mode()?;
            if e.modifiers.contains(KeyModifiers::CONTROL) && e.code == KeyCode::Char('c') {
                std::process::exit(1);
            } else {
                return Ok(());
            }
        }
    }
}

fn print_char(c: char, color: Color) -> Result<()> {
    let mut stdout = stdout();
    write!(&mut stdout, "{}", c.with(color))
}

fn clrscr(bg_color: Color) -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    execute!(stdout, style::SetBackgroundColor(bg_color))
}

impl<'a> EffectLauncher<'a> {
    pub(crate) fn new(opts: &'a CmdOptions) -> Self {
        Self {
            opts
        }
    }

    pub(crate) fn effect_exec(&self, s: String) -> Result<()> {
        let orig_col: u16 = 0;
        let mut orig_row: u16 = 0;
        let mut revealed: bool = false;

        let mut rng = thread_rng();
        let (cols, rows) = size()?;

        let (mut cur_row, mut cur_col) = cursor::position()?;
        let mut l: Vec<CharAttr> = vec![];

        let mut stdout = stdout().lock();

        clrscr(self.opts.bg_color)?;

        for c in s.chars() {
            // when output is longer than current term width, simply truncate it
            if cur_row - orig_row >= rows - 1 {
                break;
            }

            let att = CharAttr {
                src: c,
                mask: *get_random_char(),
                width: c.width().unwrap_or(0),
                is_space: (self.opts.mask_blank && c.is_whitespace()) || c == '\n',
                time: Duration::from_millis(rng.gen_range(500..5000)),
            };

            cur_col += att.width as u16;

            l.push(att);

            if c == '\n' {
                cur_col = 0;
                cur_row += 1;
                if cur_row == rows + 1 || cur_col == cols {
                    cur_col = 0;
                    cur_row += 1;
                    if cur_row == rows + 1 && orig_row > 0 {
                        orig_row -= 1;
                        cur_row -= 1;
                    }
                }
            }
        }

        stdout.execute(cursor::MoveTo(orig_row, orig_col))?;
        for att in l.iter() {
            if att.is_space {
                write!(&mut stdout, "{}", att.src)?;
                continue;
            }

            //dbg!(&att);
            write!(&mut stdout, "{}", att.mask)?;
            if att.width == 2 {
                write!(&mut stdout, "{}", get_random_char())?;
            }

            stdout.flush()?;
            std::thread::sleep(Duration::from_millis(EFFECT_SPEED as u64));
        }

        // autodecrypt todo
        if self.opts.autodecrypt {
            std::thread::sleep(Duration::from_secs(1));
        } else {
            wait_for_input()?;
        }

        // jumble
        let mut i = 0;
        while i < (JUMBLE_SECS * 1000) / JUMBLE_LOOP_SPEED {
            i += 1;
            stdout.execute(cursor::MoveTo(orig_row, orig_col))?;
            for att in l.iter() {
                if att.is_space {
                    write!(&mut stdout, "{}", att.src)?;
                    continue;
                }

                write!(&mut stdout, "{}", get_random_char())?;
                if att.width == 2 {
                    write!(&mut stdout, "{}", get_random_char())?;
                }

                stdout.flush()?;
            }
            std::thread::sleep(Duration::from_millis(JUMBLE_LOOP_SPEED as u64));
        }

        while !revealed {
            stdout.execute(cursor::MoveTo(orig_row, orig_col))?;
            revealed = true;
            for att in l.iter_mut() {
                if att.is_space {
                    write!(&mut stdout, "{}", att.src)?;
                    continue;
                }

                // If we still have time before the char is revealed, display the mask
                if !att.time.is_zero() {
                    if att.time.as_millis() < 500 {
                        if rng.gen_range(0..3) == 0_u8 {
                            att.mask = *get_random_char();
                        }
                    } else if rng.gen_range(0..10) == 0_u8 {
                        att.mask = *get_random_char();
                    }

                    write!(&mut stdout, "{}", att.mask)?;

                    att.time = att
                        .time
                        .saturating_sub(Duration::from_millis(REVEAL_LOOP_SPEED as u64));
                    revealed = false;
                } else {
                    print_char(att.src, self.opts.fg_color)?
                }

                stdout.flush()?;
            }
            std::thread::sleep(Duration::from_millis(REVEAL_LOOP_SPEED as u64));
        }

        Ok(())
    }
}
