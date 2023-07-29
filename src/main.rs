pub mod charset;
pub mod effect;
mod cmdline;

use cmdline::CmdOptions;
use effect::EffectLauncher;
use crossterm::Result;
use std::{io::{self, Read, }};



fn main() -> Result<()> {
    let opts = CmdOptions::from_args();
    let effect = EffectLauncher::new(&opts);

    let mut stdin = io::stdin();
    let mut buf = Vec::with_capacity(1024);
    stdin.read_to_end(&mut buf)?;
    effect.effect_exec(String::from_utf8_lossy(&buf).to_string())?;
    
    Ok(())

}
