mod game;

use game::Ttt;

fn main() {
    let mut g = Ttt::new();
    g.play();
}
