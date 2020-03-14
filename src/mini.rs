use crate::def::{point, FerrisData, FerrisSprite, Frame, SpriteSchedule};

pub(crate) fn ferris() -> FerrisData {
    let opening_frames = OPENING.iter().map(|s| Frame::new(s)).collect();
    let opening_moves = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
    let opening_sprite = FerrisSprite::new(opening_frames, opening_moves.iter().map(|&x| x));
    let opening_schedule = SpriteSchedule::new(0, 1);
    let run_frames = RUNNING.iter().map(|s| Frame::new(s)).collect();
    let run_moves = [(1, 0), (1, 0), (1, 0)];
    let run_sprite = FerrisSprite::new(run_frames, run_moves.iter().map(|&x| x));
    let run_schedule = SpriteSchedule::new(1, 20);
    FerrisData {
        sprites: vec![opening_sprite, run_sprite],
        schedule: vec![opening_schedule, run_schedule],
        start: point(10, 8),
    }
}

const RUNNING: [&'static str; 3] = [
    r"
    _~^~^~_
\) /  o o  \ (/
  '_   ¬   _'
  / '-----' \
",
    r"
    _~^~^~_
\) /  o o  \ (/
  '_   ¬   _'
  | '-----' |
",
    r"
    _~^~^~_
\) /  o o  \ (/
  '_   ¬   _'
  \ '-----' /
",
];

const OPENING: [&'static str; 5] = [
    r"
    _~^~^~_
\) /  o o  \ (/
  '_   ¬   _'
  \ '-----' /
",
    r"
    _~^~^~_
\) /  o o  \ (/
 '-,   -  _'\
  | '----'
",
    r"
    .~'^'^-, (/
\) /  o O  |'
 '-,   -  _'\
  | '----'
",
    r"
    .~'^'^-, (/
\) /  o O  |'
 '-,   -  _'\
  | '----'
",
    r"
    _~^~^~_
\) /  o o  \ (/
 '-,   -  _'\
  | '----'
",
];
