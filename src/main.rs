//! Basic hello world example.

extern crate ggez;
// use ggez::conf;
// use ggez::event;
// use ggez::event::{
//     Mod,
//     Keycode,
// };
// use ggez::{
//     Context,
//     GameResult
// };
// use ggez::graphics;
// use std::env;
// use std::path;
use std::collections::HashMap;
use std::collections::HashSet;

// mod input;
mod graph;

// First we make a structure to contain the game's state
// pub struct MainState {
//     font: graphics::Font,
//     text: graphics::Text,
//     frames: usize,
//     score: usize,
//     player_y: f32,
//     player_x: f32,
//     player_move_speed: f32,
//     gui_needs_update: bool,
//     player_needs_update: bool,
// }

// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
// impl MainState {
//     fn new(ctx: &mut Context) -> GameResult<MainState> {
//         let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 16)?;
//         let text = graphics::Text::new(ctx, "I can make it say anything!!!", &font)?;

//         let s = MainState {
//             font: font,
//             text: text,
//             frames: 0,
//             score: 0,
//             player_x: 0.0,
//             player_y: 0.0,
//             player_move_speed: 10.0,
//             player_needs_update: true,
//             gui_needs_update: true,
//         };
//         Ok(s)
//     }
// }

// impl event::EventHandler for MainState {
//     fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
//         input::handle_key_up(self, _ctx, keycode, _keymod, _repeat);
//     }

//     fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
//         input::handle_key_down(self, _ctx, keycode, _keymod, _repeat);
//     }

//     fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
//         Ok(())
//     }

//     fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
//         graphics::clear(ctx);
//         // Drawables are drawn from their center.
//         let dest_point = graphics::Point2::new(self.text.width() as f32 / 100.0 * self.player_x,
//                                                self.text.height() as f32 / 100.0 * self.player_y);
//         if self.player_needs_update {
//             println!("update: {}", dest_point);

//             self.text = graphics::Text::new(ctx, "@", &self.font).unwrap();
//             self.player_needs_update = false;
//         }
//         graphics::draw(ctx, &self.text, dest_point, 0.0)?;
//         graphics::present(ctx);
//         // self.frames += 1;
//         // if (self.frames % 100) == 0 {
//         //     println!("FPS: {}", ggez::timer::get_fps(ctx));
//         // }
//         Ok(())
//     }
// }

// Now our main function, which does three things:
//
// * First, create a new `ggez::conf::Conf`
// object which contains configuration info on things such
// as screen resolution and window title,
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game,
// * then just call `game.run()` which runs the `Game` mainloop.
pub fn main() {
    // let c = conf::Conf::new();
    // let ctx = &mut Context::load_from_conf("helloworld", "ggez", c).unwrap();

    // // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // // we we look in the cargo project for files.
    // if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    //     let mut path = path::PathBuf::from(manifest_dir);
    //     path.push("resources");
    //     ctx.filesystem.mount(&path, true);
    // }

    // let state = &mut MainState::new(ctx).unwrap();
    // if let Err(e) = event::run(ctx, state) {
    //     println!("Error encountered: {}", e);
    // } else {
    //     println!("Game exited cleanly.");
    // }
    let mut example_graph = graph::Graph {
        edges: HashMap::new()
    };
    example_graph.edges.insert('A', vec!['B']);
    example_graph.edges.insert('B', vec!['A', 'C', 'D']);
    example_graph.edges.insert('C', vec!['A']);
    example_graph.edges.insert('D', vec!['E', 'A']);
    example_graph.edges.insert('E', vec!['B']);

    graph::Graph::breadth_first_search_1(&example_graph, 'A');

    let mut example_grid = graph::SquareGrid {
        width: 30,
        height: 15,
        walls: HashSet::new(),
    };

    build_debug_grid_walls(&mut example_grid);

    let parents = example_grid.breadth_first_search_2(graph::Point{x: 16, y: 7});
    // draw_grid(g, width=2, point_to=parents, start=(8, 7))
    example_grid.draw_grid(2, &parents);
}

fn build_debug_grid_walls(graph: &mut graph::SquareGrid) {
    graph.walls.insert(graph::Point{x: 6, y: 2});
    graph.walls.insert(graph::Point{x: 6, y: 3});
    graph.walls.insert(graph::Point{x: 6, y: 4});
    graph.walls.insert(graph::Point{x: 6, y: 5});
    graph.walls.insert(graph::Point{x: 6, y: 6});
    graph.walls.insert(graph::Point{x: 6, y: 7});
    graph.walls.insert(graph::Point{x: 6, y: 8});
    graph.walls.insert(graph::Point{x: 6, y: 9});

    graph.walls.insert(graph::Point{x: 14, y: 4});
    graph.walls.insert(graph::Point{x: 14, y: 5});
    graph.walls.insert(graph::Point{x: 14, y: 6});
    graph.walls.insert(graph::Point{x: 14, y: 7});
    graph.walls.insert(graph::Point{x: 14, y: 8});
    graph.walls.insert(graph::Point{x: 14, y: 9});
    graph.walls.insert(graph::Point{x: 14, y: 10});
    graph.walls.insert(graph::Point{x: 14, y: 11});
    graph.walls.insert(graph::Point{x: 14, y: 12});
    graph.walls.insert(graph::Point{x: 14, y: 13});

    graph.walls.insert(graph::Point{x: 22, y: 2});
    graph.walls.insert(graph::Point{x: 22, y: 3});
    graph.walls.insert(graph::Point{x: 22, y: 4});
    graph.walls.insert(graph::Point{x: 22, y: 5});
    graph.walls.insert(graph::Point{x: 22, y: 6});
    graph.walls.insert(graph::Point{x: 22, y: 7});
    graph.walls.insert(graph::Point{x: 22, y: 8});
    graph.walls.insert(graph::Point{x: 22, y: 9});
}