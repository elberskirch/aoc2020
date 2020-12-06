use std::env;
use std::fs;


// traverse through a given/virtually repeating grid 
struct Cursor {
    x: u32,
    y: u32,
    x_max: u32,
    y_max: u32,
}

impl Cursor {
    fn mov(&mut self) -> (u32,u32) {
        self.x =  (self.x + 3) % self.x_max;
        self.y += 1;

        (self.x,self.y)
    } 

    fn last_line(&self) -> bool {
        self.y == self.y_max 
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
    // ...
    //
   
    let mut grid: Vec<Vec<char>> = Vec::new();
    for l in lines {
        let c = l.chars().collect::<Vec<char>>();
        println!("{:?}", c);
        grid.push(c);
    }
    
    println!("x_max: {} y_max: {}", grid[0].len(), grid.len());
    
    // setup cursor to move through grid
    let mut cursor = Cursor {x: 0,y: 0, x_max: (grid[0].len() as u32), y_max: (grid.len() as u32)};
    let mut trees: i32 = 0;

    //for grid_line in grid {
     while !cursor.last_line() {
        let pos = cursor.mov();
        if cursor.last_line() {break;}
         
        //println!("pos.0 = {} - {:?}", pos.0, grid_line);
        if grid[(pos.1 as usize)][(pos.0 as usize)] == '#' { 
            println!("--- MATCH cursor x = {} - y = {}", pos.0, pos.1);
            trees += 1; 
        }
    }
    println!("List of trees crossed: {}", trees);
}
