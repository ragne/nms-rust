use crossterm::style::Color;
use getopts::Options;
use std::{env, str::FromStr};

fn print_usage_and_exit(program: &str, opts: Options) -> ! {
    let brief = format!("Usage: {program}");
    print!("{}", opts.usage(&brief));

    std::process::exit(1);
}

#[derive(Debug)]
pub(crate) struct CmdOptions {
    pub(crate) autodecrypt: bool,
    pub(crate) mask_blank: bool,
    pub(crate) bg_color: Color,
    pub(crate) fg_color: Color,
}

impl CmdOptions {
    pub(crate) fn from_args() -> Self {
        let args: Vec<String> = env::args().collect();
        let program = args[0].clone();

        let mut opts = Options::new();
        opts.optopt("b", "background", "sets background color", "COLOR");
        opts.optopt("c", "color", "sets foreground color", "COLOR");
        opts.optflag(
            "s",
            "mask-blank",
            "if enabled then spaces are encrypted too",
        );
        opts.optflag("h", "help", "print this help menu");
        opts.optflag("a", "autodecrypt", "if set enables autodecrypt, you don't have to press a key for start the decryption process");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(_) => print_usage_and_exit(&program, opts),
        };

        if matches.opt_present("h") {
            print_usage_and_exit(&program, opts);
        }

        let bg_color = matches
            .opt_str("b")
            .map(|c| Color::from_str(&c).unwrap())
            .unwrap_or(Color::Reset);
        let fg_color = matches
            .opt_str("c")
            .map(|c| Color::try_from(c.as_str()).unwrap_or(Color::Blue))
            .unwrap_or(Color::Blue);

        Self {
            autodecrypt: matches.opt_present("a"),
            mask_blank: !matches.opt_present("s"),
            bg_color,
            fg_color,
        }
    }
}
