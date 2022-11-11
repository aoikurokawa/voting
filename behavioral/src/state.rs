use cursive::{event::Key, views::TextView, Cursive};

pub struct Track {
    pub title: String,
    pub duration: u32,
    cursor: u32,
}

impl Track {
    pub fn new(title: &'static str, duration: u32) -> Self {
        Self {
            title: title.into(),
            duration,
            cursor: 0,
        }
    }
}

pub struct Player {
    playlist: Vec<Track>,
    current_stack: usize,
    _volume: u8,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            playlist: vec![
                Track::new("Track 1", 180),
                Track::new("Track 2", 165),
                Track::new("Track 3", 197),
                Track::new("Track 4", 205),
            ],
            current_stack: 0,
            _volume: 25,
        }
    }
}

impl Player {
    pub fn next_track(&mut self) {
        self.current_stack = (self.current_stack + 1) % self.playlist.len();
    }

    pub fn prev_track(&mut self) {
        self.current_stack = (self.playlist.len() + self.current_stack - 1) % self.playlist.len();
    }

    pub fn play(&mut self) {
        self.track_mut().cursor = 10;
    }

    pub fn pause(&mut self) {
        self.track_mut().cursor = 43;
    }

    pub fn rewind(&mut self) {
        self.track_mut().cursor = 0;
    }

    pub fn track(&self) -> &Track {
        &self.playlist[self.current_stack]
    }

    fn track_mut(&mut self) -> &mut Track {
        &mut self.playlist[self.current_stack]
    }
}

pub struct StoppedState;
pub struct PausedState;
pub struct PlayingState;

pub trait State {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State>;
    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State>;
    fn render(&self, player: &Player, view: &mut TextView);
}

impl State for StoppedState {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.play();

        // Stopped -> Playing
        Box::new(PlayingState)
    }

    fn stop(self: Box<Self>, _: &mut Player) -> Box<dyn State> {
        self
    }

    fn render(&self, _player: &Player, view: &mut TextView) {
        view.set_content("[Stopped] Press 'Play'")
    }
}

impl State for PausedState {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.pause();

        Box::new(PlayingState)
    }

    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.pause();
        player.rewind();

        Box::new(StoppedState)
    }

    fn render(&self, player: &Player, view: &mut TextView) {
        view.set_content(format!(
            "[Paused] {} - {} sec",
            player.track().title,
            player.track().duration,
        ))
    }
}

impl State for PlayingState {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.pause();

        Box::new(PlayingState)
    }

    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.pause();
        player.rewind();

        Box::new(StoppedState)
    }

    fn render(&self, player: &Player, view: &mut TextView) {
        view.set_content(format!(
            "[Playing] {} - {} sec",
            player.track().title,
            player.track().duration,
        ))
    }
}

impl dyn State {
    pub fn next(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.next_track();

        self
    }

    pub fn prev(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.prev_track();

        self
    }
}

pub struct PlayerApplication {
    player: Player,
    state: Box<dyn State>,
}

pub fn execute(s: &mut Cursive, button: &'static str) {
    let PlayerApplication {
        mut player,
        mut state,
    } = s.take_user_data().unwrap();

    let mut view = s.find_name::<TextView>("Player Status").unwrap();

    state = match button {
        "Play" => state.play(&mut player),
        "Stop" => state.stop(&mut player),
        "Prev" => state.prev(&mut player),
        "Next" => state.next(&mut player),
        _ => unreachable!(),
    };

    state.render(&player, &mut view);

    s.set_user_data(PlayerApplication { player, state });
}

#[cfg(test)]
mod tests {
    use cursive::{view::Nameable, views::Dialog};

    use super::*;

    #[test]
    fn test_state() {
        let mut app = cursive::default();

        app.set_user_data(PlayerApplication {
            player: Player::default(),
            state: Box::new(StoppedState),
        });

        app.add_layer(
            Dialog::around(TextView::new("Press Play").with_name("Player Status"))
                .title("Music Player")
                .button("Play", |s| execute(s, "Play"))
                .button("Stop", |s| execute(s, "Stop"))
                .button("Prev", |s| execute(s, "Prev"))
                .button("Next", |s| execute(s, "Next")),
        );

        app.add_global_callback(Key::Esc, |s| s.quit());

        app.run();
    }
}
