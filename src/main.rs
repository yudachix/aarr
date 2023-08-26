use crossterm::{cursor, terminal, ExecutableCommand};
use serde::Deserialize;
use std::io::{self, stdout, Write};
use std::thread;
use std::time::Duration;

#[derive(Deserialize)]
struct Frame {
    delay: u32,
    content: String,
}

fn main() -> io::Result<()> {
    let mut stdout = stdout();
    let s = include_str!("../dist/frames.json");
    let frames: Vec<Frame> = serde_json::from_str(s)?;

    loop {
        for (i, frame) in frames.iter().enumerate() {
            let i = if i > 0 { i } else { frames.len() };
            if let Some(prev) = frames.get(i - 1) {
                let lines = prev.content.split('\n').collect::<Vec<_>>();

                stdout.execute(cursor::MoveUp(lines.len() as u16 + 2))?;
                stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?;
            }

            writeln!(stdout, "{}\nCtrl + C to exit", frame.content)?;
            thread::sleep(Duration::from_millis(frame.delay as u64));
        }
    }
}
