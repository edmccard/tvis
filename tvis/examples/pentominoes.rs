#[macro_use]
extern crate lazy_static;

extern crate dlx;
extern crate tvis;

use std::sync::mpsc::{channel, Receiver};
use tvis::input::{Event, InputEvent, Key};
use tvis::term::{self, BoldOrBright, Color, Style, Terminal, UseTruecolor};

fn main() {
    let (tx, rx) = channel();
    let mut screen = term::connect_with_input(
        tx.clone(),
        UseTruecolor::Auto,
        BoldOrBright::Bright,
    ).unwrap();
    if !screen.is_tty_input() || !screen.is_tty_output() {
        screen.log("input or output is not a terminal");
        return;
    }

    let grid = ChessGrid::new(3, 3);
    let mut solver = dlx::Solver::new(72, PentMatrix::new(&grid));
    let mut solutions = StaticSolutions::new(&grid, screen, rx);
    solver.solve(vec![], &mut solutions);
}

struct StaticSolutions<'a> {
    grid: &'a Grid,
    screen: Box<Terminal>,
    rx: Receiver<Box<Event>>,
    done: bool,
}

impl<'a> StaticSolutions<'a> {
    fn new(
        grid: &Grid,
        mut screen: Box<Terminal>,
        rx: Receiver<Box<Event>>,
    ) -> StaticSolutions {
        screen.cursor_visible(false).unwrap();
        screen.start_input().unwrap();
        StaticSolutions {
            grid,
            screen,
            rx,
            done: false,
        }
    }

    fn visualize(&self, sol: Vec<dlx::Row>) -> Vec<Vec<Color>> {
        let mut vis =
            vec![vec![Color::Default; self.grid.width()]; self.grid.height()];
        for row in sol {
            let color = Color::Palette((row[0] + 1) as u8);
            for r in &row[1..] {
                let (x, y) = self.grid.col_to_pos(*r - 12);
                vis[y][x] = color;
            }
        }
        vis
    }

    fn paint(&mut self, sol: &Vec<Vec<Color>>) {
        self.screen.clear().unwrap();
        let size = self.screen.get_size().unwrap();
        if size.rows < 16 || size.cols < 24 {
            return;
        }
        let cx = (size.cols - 24) / 2;
        let cy = (size.rows - 16) / 2;
        for y in 0..8 {
            for line in 0..2 {
                self.screen.set_cursor((cx, cy + (y * 2 + line))).unwrap();
                for x in 0..8 {
                    self.screen
                        .set_style(
                            Style::empty(),
                            Color::Default,
                            sol[y as usize][x as usize],
                        )
                        .unwrap();
                    self.screen.write("   ").unwrap();
                }
            }
        }
        self.screen
            .set_style(Style::empty(), Color::Default, Color::Default)
            .unwrap();
        self.screen.flush_output().unwrap();
    }
}

impl<'a> dlx::Solutions for StaticSolutions<'a> {
    fn push(&mut self, sol: dlx::Solution) -> bool {
        let vis = self.visualize(sol.collect::<Vec<_>>());
        self.paint(&vis);
        loop {
            let evt = match self.rx.recv() {
                Err(_) => {
                    self.done = true;
                    return false;
                }
                Ok(evt) => evt,
            };
            if let Some(evt) = evt.as_any().downcast_ref::<InputEvent>() {
                match *evt {
                    InputEvent::Key(Key::Esc, _) => {
                        self.done = true;
                        return false;
                    }
                    InputEvent::Key(Key::Char(' ', _, _), _) => return true,
                    InputEvent::Repaint => self.paint(&vis),
                    _ => (),
                }
            }
        }
    }
}

type Pos = (dlx::Index, dlx::Index);
type Shape = [Pos; 5];
type Piece = (dlx::Index, Shape);

trait Grid {
    fn try_piece(&self, pos: Pos, shape: Shape) -> bool;
    fn pos_to_col(&self, pos: Pos) -> dlx::Index;
    fn col_to_pos(&self, col: dlx::Index) -> Pos;
    fn width(&self) -> dlx::Index;
    fn height(&self) -> dlx::Index;
}

struct ChessGrid {
    hole_x: dlx::Index,
    hole_y: dlx::Index,
}

impl ChessGrid {
    fn new(hole_x: dlx::Index, hole_y: dlx::Index) -> ChessGrid {
        if hole_x >= 7 || hole_y >= 7 {
            panic!();
        }
        ChessGrid { hole_x, hole_y }
    }

    fn pos_ok(&self, pos: Pos) -> bool {
        let (x, y) = pos;
        if x < 8 && y < 8 {
            if (x == self.hole_x || x == self.hole_x + 1)
                && (y == self.hole_y || y == self.hole_y + 1)
            {
                return false;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }
}

impl Grid for ChessGrid {
    fn try_piece(&self, pos: Pos, shape: Shape) -> bool {
        for offset in shape.iter() {
            if !self.pos_ok((pos.0 + offset.0, pos.1 + offset.1)) {
                return false;
            }
        }
        true
    }

    fn pos_to_col(&self, pos: Pos) -> dlx::Index {
        if !self.pos_ok(pos) {
            panic!();
        }
        let col = pos.1 * 8 + pos.0;
        if col > (self.hole_y * 8 + self.hole_x + 1) {
            if col < ((self.hole_y + 1) * 8 + self.hole_x) {
                return col - 2;
            } else {
                return col - 4;
            }
        }
        col
    }

    fn col_to_pos(&self, col: dlx::Index) -> Pos {
        let col = if col >= (self.hole_y * 8 + self.hole_x) {
            if col <= (((self.hole_y + 1) * 8 + self.hole_x + 1) - 4) {
                col + 2
            } else {
                col + 4
            }
        } else {
            col
        };
        (col % 8, col / 8)
    }

    fn width(&self) -> dlx::Index {
        8
    }

    fn height(&self) -> dlx::Index {
        8
    }
}

struct PentMatrix<'a> {
    grid: &'a Grid,
    piece_idx: dlx::Index,
    x: dlx::Index,
    y: dlx::Index,
}

impl<'a> PentMatrix<'a> {
    fn new(grid: &Grid) -> PentMatrix {
        PentMatrix {
            grid,
            piece_idx: 0,
            x: 0,
            y: 0,
        }
    }
}

impl<'a> Iterator for PentMatrix<'a> {
    type Item = dlx::Row;

    fn next(&mut self) -> Option<Self::Item> {
        let width = self.grid.width();
        loop {
            if self.x >= width {
                self.x = 0;
                self.y += 1;
            }
            if self.y >= self.grid.height() {
                self.y = 0;
                self.piece_idx += 1;
            }
            if self.piece_idx >= PIECES.len() {
                return None;
            }
            let piece = PIECES[self.piece_idx];
            let ok = self.grid.try_piece((self.x, self.y), piece.1);
            if ok {
                let mut row = vec![piece.0 - 1];
                for offset in piece.1.iter() {
                    let x = self.x + offset.0;
                    let y = self.y + offset.1;
                    row.push(self.grid.pos_to_col((x, y)) + 12);
                }
                self.x += 1;
                return Some(row);
            } else {
                self.x += 1;
            }
        }
    }
}

lazy_static! {
    static ref PIECES: Vec<Piece> = vec![
        // F
        (1, [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)]),
        (1, [(1, 0), (0, 1), (1, 1), (2, 1), (2, 2)]),
        (1, [(1, 0), (1, 1), (2, 1), (0, 2), (1, 2)]),
        (1, [(0, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        (1, [(0, 0), (1, 0), (1, 1), (2, 1), (1, 2)]),
        (1, [(1, 0), (0, 1), (1, 1), (2, 1), (0, 2)]),
        (1, [(1, 0), (0, 1), (1, 1), (1, 2), (2, 2)]),
        (1, [(2, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        // I
        (2, [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)]),
        (2, [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]),
        // L
        (3, [(0, 0), (0, 1), (0, 2), (0, 3), (1, 3)]),
        (3, [(0, 0), (1, 0), (2, 0), (3, 0), (0, 1)]),
        (3, [(0, 0), (1, 0), (1, 1), (1, 2), (1, 3)]),
        (3, [(3, 0), (0, 1), (1, 1), (2, 1), (3, 1)]),
        (3, [(1, 0), (1, 1), (1, 2), (0, 3), (1, 3)]),
        (3, [(0, 0), (1, 0), (2, 0), (3, 0), (3, 1)]),
        (3, [(0, 0), (1, 0), (0, 1), (0, 2), (0, 3)]),
        (3, [(0, 0), (0, 1), (1, 1), (2, 1), (3, 1)]),
        // N
        (4, [(1, 0), (1, 1), (0, 2), (1, 2), (0, 3)]),
        (4, [(0, 0), (1, 0), (1, 1), (2, 1), (3, 1)]),
        (4, [(1, 0), (0, 1), (1, 1), (0, 2), (0, 3)]),
        (4, [(0, 0), (1, 0), (2, 0), (2, 1), (3, 1)]),
        (4, [(0, 0), (0, 1), (0, 2), (1, 2), (1, 3)]),
        (4, [(2, 0), (3, 0), (0, 1), (1, 1), (2, 1)]),
        (4, [(0, 0), (0, 1), (1, 1), (1, 2), (1, 3)]),
        (4, [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1)]),
        // P
        (5, [(0, 0), (1, 0), (0, 1), (1, 1), (0, 2)]),
        (5, [(0, 0), (1, 0), (2, 0), (1, 1), (2, 1)]),
        (5, [(1, 0), (0, 1), (1, 1), (0, 2), (1, 2)]),
        (5, [(0, 0), (1, 0), (0, 1), (1, 1), (2, 1)]),
        (5, [(0, 0), (1, 0), (0, 1), (1, 1), (1, 2)]),
        (5, [(0, 0), (1, 0), (2, 0), (0, 1), (1, 1)]),
        (5, [(0, 0), (0, 1), (1, 1), (0, 2), (1, 2)]),
        (5, [(1, 0), (2, 0), (0, 1), (1, 1), (2, 1)]),
        // T
        (6, [(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)]),
        (6, [(2, 0), (0, 1), (1, 1), (2, 1), (2, 2)]),
        (6, [(1, 0), (1, 1), (0, 2), (1, 2), (2, 2)]),
        (6, [(0, 0), (0, 1), (1, 1), (2, 1), (0, 2)]),
        // U
        (7, [(0, 0), (2, 0), (0, 1), (1, 1), (2, 1)]),
        (7, [(0, 0), (1, 0), (0, 1), (0, 2), (1, 2)]),
        (7, [(0, 0), (1, 0), (2, 0), (0, 1), (2, 1)]),
        (7, [(0, 0), (1, 0), (1, 1), (0, 2), (1, 2)]),
        // V
        (8, [(2, 0), (2, 1), (0, 2), (1, 2), (2, 2)]),
        (8, [(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)]),
        (8, [(0, 0), (1, 0), (2, 0), (0, 1), (0, 2)]),
        (8, [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
        // W
        (9, [(2, 0), (1, 1), (2, 1), (0, 2), (1, 2)]),
        (9, [(0, 0), (0, 1), (1, 1), (1, 2), (2, 2)]),
        (9, [(1, 0), (2, 0), (0, 1), (1, 1), (0, 2)]),
        (9, [(0, 1), (1, 0), (1, 1), (2, 1), (2, 2)]),
        // X
        (10, [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        // Y
        (11, [(1, 0), (0, 1), (1, 1), (1, 2), (1, 3)]),
        (11, [(2, 0), (0, 1), (1, 1), (2, 1), (3, 1)]),
        (11, [(0, 0), (0, 1), (0, 2), (1, 2), (0, 3)]),
        (11, [(0, 0), (1, 0), (2, 0), (3, 0), (1, 1)]),
        (11, [(0, 0), (0, 1), (1, 1), (0, 2), (0, 3)]),
        (11, [(1, 0), (0, 1), (1, 1), (2, 1), (3, 1)]),
        (11, [(1, 0), (1, 1), (0, 2), (1, 2), (1, 3)]),
        (11, [(0, 0), (1, 0), (2, 0), (3, 0), (2, 1)]),
        // Z
        (12, [(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)]),
        (12, [(2, 0), (0, 1), (1, 1), (2, 1), (0, 2)]),
        (12, [(1, 0), (2, 0), (1, 1), (0, 2), (1, 2)]),
        (12, [(0, 0), (0, 1), (1, 1), (2, 1), (2, 2)]),
    ];
}
