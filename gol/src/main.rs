use std::io;
use std::io::Write;

use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::time::{Instant}; // Duration
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
struct Cell {
    x: i32,
    y: i32
}
impl Cell {
    fn alive(&self, board: &HashSet::<Cell>) -> bool {
        let nbs = self.neighbours().filter(|c| board.contains(c)).count();
        board.contains(self) && nbs == 2 || nbs == 3
    }
    fn neighbours(&self) -> impl Iterator<Item=Cell>+'_ {
        (-1..=1).flat_map(
            move |x| (-1..=1).filter(move |y| !(x==0 && *y==0)).map(move |y| Cell {x:self.x + x, y:self.y + y}))
        // .collect()
    }
}
fn next(board: &HashSet::<Cell>) -> HashSet::<Cell> {
    board.into_iter().flat_map(|c| c.neighbours()).unique().filter(|c| c.alive(board)).collect()
}
fn render(board: &HashSet::<Cell>) {
    let stdout = io::stdout(); 
    let mut w = io::BufWriter::new(stdout);
    const min:Cell = Cell{x:0,y:0};
    const max:Cell = Cell{x:40,y:40};
    let x0 = board.iter().min_by_key(|c| c.x).unwrap_or(&min).x-5;
    let y0 = board.iter().min_by_key(|c| c.y).unwrap_or(&min).y-5;
    let x1 = board.iter().max_by_key(|c| c.x).unwrap_or(&max).x+5;
    let y1 = board.iter().max_by_key(|c| c.y).unwrap_or(&max).y+5;
    print!("\x1b[2;1H");
    for x in x0..=x1 {
        for y in y0..=y1 {
            write!(w, "{}", if board.contains(&Cell{x:x, y:y}) {'X'} else {'.'}).unwrap();
        }
        writeln!(w).unwrap();
        // writeln!(w, "{}", (y0..=y1).map( |y| if board.contains(&Cell{x:x, y:y}) {"X"} else {"."}).collect::<String>());
    }
}

fn fps(start:&Instant, count:&u32 /*AtomicUsize*/) {
    let duration = start.elapsed().as_secs();
    let c = *count; // count.load(Ordering::Relaxed);
    let fps = if duration > 0 {c as f32 / duration as f32 } else { c as f32};
    print!("{} frames took {} seconds, rendering {} fps", c, duration, fps);
}

fn main() {
    let mut count = 0; // AtomicUsize::new(0);
    let start = Instant::now();
    ctrlc::set_handler(move || {
        println!();
        fps(&start, &count);
        println!();
        std::process::exit(exitcode::OK);
    }).expect("Error setting Ctrl-C handler");

    // let cell = Cell{x:0,y:0};
    let mut board = HashSet::from_iter(vec![[1,0],[2,0],[0,1],[1,1],[1,2]].iter().map(|t| Cell{x:t[0], y:t[1]}));
    // println!("cells {:?} alive {} contains {}", board, &cell.alive(&board), &board.contains(&Cell {x:0, y:0}));
    // println!("neighbours {:?}", &cell.neighbours());
    // println!("next {:?}", next(&board));
    print!("\x1B[2J");
    for round in 0..10000 {
        render(&board);
        board = next(&board);
        if round % 10 == 0 {
            print!("\x1b[1;1H");
            count = round;
            // count.store(round, Ordering::Relaxed);
            fps(&start, &count);
        }
    }
}
