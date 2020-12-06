use std::env;
use std::fs;


// traverse through a given/virtually repeating grid 
struct Cursor {
    x: u32,
    y: u32,
    x_max: u32,
    y_max: u32,
    x_slope: u32,
    y_slope: u32,
}

impl Cursor {
    fn mov(&mut self) -> (u32,u32) {
        self.x =  (self.x + self.x_slope) % self.x_max;
        self.y += self.y_slope;

        (self.x,self.y)
    } 

    fn last_line(&self) -> bool {
        self.y >= self.y_max 
    }
}

struct MapHandler {
    cursor: Cursor,
    trees: i32,
}

fn check_pos(grid: &mut Vec<Vec<char>>, x: u32, y: u32, trees: &mut i32) {
    if grid[(y as usize)][(x as usize)] == '#' { 
        println!("--- MATCH cursor x = {} - y = {}", x, y);
        *trees += 1; 
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];

    let contents = fs::read_to_string(input_filename)
        .expect("Something went wrong reading the file");

    let mut lines: Vec<&str> = contents.split("\n").collect();
   
    lines.pop(); // get rid of empty last element
    println!("Number of lines: {}", lines.len());
    
    // create two-dimensional grid of characters
    let mut grid: Vec<Vec<char>> = Vec::new();
    for l in lines {
        let c = l.chars().collect::<Vec<char>>();
        println!("{:?}", c);
        grid.push(c);
    }
    
    println!("x_max: {} y_max: {}", grid[0].len(), grid.len());
    
    // setup cursors to move through grid
    let mut cursors = [
        MapHandler{ cursor: Cursor {x: 0,y: 0, 
            x_max: (grid[0].len() as u32), y_max: (grid.len() as u32), 
            x_slope: 1, y_slope: 1 }, trees: 0},
        MapHandler{ cursor: Cursor {x: 0,y: 0, 
            x_max: (grid[0].len() as u32), y_max: (grid.len() as u32), 
            x_slope: 3, y_slope: 1 }, trees: 0},
        MapHandler{ cursor: Cursor {x: 0,y: 0, 
            x_max: (grid[0].len() as u32), y_max: (grid.len() as u32), 
            x_slope: 5, y_slope: 1 }, trees: 0},
        MapHandler{ cursor: Cursor {x: 0,y: 0, 
            x_max: (grid[0].len() as u32), y_max: (grid.len() as u32), 
            x_slope: 7, y_slope: 1 }, trees: 0},
        MapHandler{ cursor: Cursor {x: 0,y: 0, 
            x_max: (grid[0].len() as u32), y_max: (grid.len() as u32), 
            x_slope: 1, y_slope: 2 }, trees: 0},
    ];
    
    let mut res: i32 = 1;
    // dirty/lazy copy paste 
    for c in &mut cursors {
        println!("x_slope {} - y_slope {}", c.cursor.x_slope, c.cursor.y_slope );
        while !c.cursor.last_line() {
            let pos = c.cursor.mov();
            if c.cursor.last_line() {break;}
            check_pos(&mut grid, pos.0, pos.1, &mut c.trees);
        }
        println!("List of trees crossed: {}", c.trees);
        res *= c.trees;
    }
    println!("Result: {}", res);

}
