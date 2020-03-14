use anyhow::Result;
use crossterm::{queue, style::Print, terminal, ExecutableCommand};
use std::{
    io::{stdout, Write},
    thread, time,
};
use structopt::StructOpt;

mod def;
mod mini;
use def::{Command, FerrisCommands};

#[derive(StructOpt, Debug)]
#[structopt(name = "fsl")]
struct Cli {
    #[structopt(short = "s", long)]
    style: Option<String>,
}

impl Cli {
    fn run(self) -> Result<()> {
        let Cli { style } = self;
        let style = style.as_deref().unwrap_or("mini");
        let ferris = match style {
            "mini" => mini::ferris(),
            _ => anyhow::bail!("Unsupported style: {}", style),
        };
        let mut stdout = stdout();
        for commands in FerrisCommands::new(&ferris) {
            stdout.execute(terminal::Clear(terminal::ClearType::All))?;
            for cmd in commands {
                match cmd {
                    Command::MoveTo(m) => queue!(stdout, m),
                    Command::Print(s) => queue!(stdout, Print(s)),
                }?;
            }
            stdout.flush()?;
            thread::sleep(time::Duration::from_millis(100));
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    Cli::from_args().run()
}
