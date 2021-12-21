use eframe::{egui, epi};
use egui_inspect::Inspect;
use egui_inspect_derive::Inspect;
use rand::{distributions::Alphanumeric, prelude::SliceRandom, thread_rng, Rng};

struct Testbed {
    entities: Vec<GameEntity>,
}

#[derive(Inspect)]
struct GameEntity {
    name: String,
    position: Vector2,
    hp: i32,
    godmode: bool,
    dir: Dir,
    #[opaque]
    #[allow(dead_code)]
    something_opaque: MyOpaque,
}

struct MyOpaque;

#[derive(Inspect, Clone, Copy, PartialEq)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl GameEntity {
    fn rand() -> Self {
        let mut rng = rand::thread_rng();
        let name_len = rng.gen_range(3..24);
        Self {
            name: (&mut rng)
                .sample_iter(&Alphanumeric)
                .take(name_len)
                .map(char::from)
                .collect(),
            position: Vector2::rand(),
            hp: rng.gen_range(0..100),
            godmode: rng.gen(),
            dir: *[Dir::North, Dir::East, Dir::South, Dir::West]
                .choose(&mut rng)
                .unwrap(),
            something_opaque: MyOpaque,
        }
    }
}

#[derive(Inspect)]
struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    fn rand() -> Self {
        let mut rng = thread_rng();
        Self {
            x: rng.gen(),
            y: rng.gen(),
        }
    }
}

impl Default for Testbed {
    fn default() -> Self {
        Self {
            entities: (0..100).map(|_| GameEntity::rand()).collect(),
        }
    }
}

impl epi::App for Testbed {
    fn name(&self) -> &str {
        "egui-inspect testbed"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.entities.inspect(ui, 0);
            })
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(Testbed::default()), options);
}
