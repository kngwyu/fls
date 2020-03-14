use crossterm::cursor::MoveTo;

#[derive(Clone, Debug)]
pub(crate) enum Command {
    MoveTo(MoveTo),
    Print(String),
}

impl From<Point> for Command {
    fn from(p: Point) -> Self {
        Command::MoveTo(MoveTo(p.x as _, p.y as _))
    }
}

pub(crate) struct ScrrenSpace;
pub(crate) type Point = euclid::Point2D<i32, ScrrenSpace>;
pub(crate) const fn point(x: i32, y: i32) -> Point {
    euclid::point2(x, y)
}

#[derive(Clone, Debug)]
pub(crate) struct FerrisData {
    pub(crate) sprites: Vec<FerrisSprite>,
    pub(crate) schedule: Vec<SpriteSchedule>,
    pub(crate) start: Point,
}

pub(crate) struct FerrisCommands<'a> {
    data: &'a FerrisData,
    position: Point,
    schedule_iter: usize,
    schedule_repeated: usize,
    frame_iter: usize,
}

impl<'a> FerrisCommands<'a> {
    pub(crate) fn new(data: &'a FerrisData) -> Self {
        FerrisCommands {
            data,
            position: data.start,
            schedule_iter: 0,
            schedule_repeated: 0,
            frame_iter: 0,
        }
    }
}

impl<'a> Iterator for FerrisCommands<'a> {
    type Item = Vec<Command>;
    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.position;
        let schedule = self.data.schedule.get(self.schedule_iter)?;
        let mut res = vec![];
        let sprite = self
            .data
            .sprites
            .get(schedule.sprite_id)
            .expect("Invalid sprite id");
        if self.frame_iter >= sprite.moves.len() {
            if self.schedule_repeated + 1 == schedule.repeat {
                // Use next sprite
                self.schedule_iter += 1;
                return self.next();
            } else {
                // Repeat the sprite
                self.frame_iter = 0;
                self.schedule_repeated += 1;
                return self.next();
            }
        }
        let move_ = sprite.moves[self.frame_iter];
        let nxt = point(cur.x + move_.x, cur.y + move_.y);
        res.push(Command::from(nxt));
        for (i, line) in sprite.frames[self.frame_iter].lines.iter().enumerate() {
            res.push(Command::Print(line.clone()));
            res.push(Command::from(point(nxt.x, nxt.y + i as i32)));
        }
        self.frame_iter += 1;
        self.position = nxt;
        Some(res)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FerrisSprite {
    frames: Vec<Frame>,
    moves: Vec<Point>,
}

impl FerrisSprite {
    pub(crate) fn new(frames: Vec<Frame>, moves: impl Iterator<Item = (i32, i32)>) -> Self {
        let moves = moves.map(|p| point(p.0, p.1)).collect::<Vec<_>>();
        assert_eq!(frames.len(), moves.len());
        FerrisSprite { frames, moves }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct SpriteSchedule {
    sprite_id: usize,
    repeat: usize,
}

impl SpriteSchedule {
    pub(crate) fn new(sprite_id: usize, repeat: usize) -> Self {
        SpriteSchedule { sprite_id, repeat }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Frame {
    lines: Vec<String>,
}

impl Frame {
    pub(crate) fn new<'a>(data: &'a str) -> Self {
        let mut res = vec![];
        let mut initial = false;
        for s in data.lines() {
            if initial && s.len() == 0 {
                continue;
            }
            initial = true;
            res.push(s.to_string());
        }
        Frame { lines: res }
    }
}
