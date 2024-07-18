use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::io::{stdin};
use rand::prelude::*;
use colored::{Colorize, ColoredString};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Alive(u8),
    Dead
}

enum Command{
    Pause,
    Redraw,
    HigherRatio,
    LowerRatio,
    Faster,
    Slower
}


const WIDTH: usize = 20;
type Board = [Cell; WIDTH * WIDTH];
type Neighbours = [Cell; 8];


fn main() {

    let (msg_sender, msg_receiver) = mpsc::channel::<Command>();
    let _handle = thread::spawn(move || {
        let mut active = true;
        let mut frame = 0;
        let mut ratio = 0.7;
        let mut board = generate_board(ratio);
        let mut sleep = 700;

        loop {
            if active {
                frame += 1;
                print!("\x1B[2J\x1B[1;1H");
                println!("frame: {} - ratio: {} - sleep duration(ms): {}", frame, ratio, sleep);
                draw_board(&board);
                println!("commands:");
                println!("r - redraw, m - increase ratio, l - lower ratio");
                println!("z - slower, x - faster, p - pause, q - quit");
                println!("you have to press enter for commands to work!");
                board = get_updated_board(&board);
            }

            let msg = msg_receiver.try_recv();
            match msg {
                Ok(Command::Pause) => { active = !active },
                Ok(Command::Redraw) => { board = generate_board(ratio); }
                Ok(Command::HigherRatio) => { ratio += 0.05; }
                Ok(Command::LowerRatio) => { ratio -= 0.05; }
                Ok(Command::Faster) => { sleep -= 50; }
                Ok(Command::Slower) => { sleep += 50; }
                _ => ()
            }
            thread::sleep(Duration::from_millis(sleep));
        }
    });

    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.as_str().trim() {
                    "q" => break,
                    "p" => { let _ = msg_sender.send(Command::Pause); },
                    "r" => { let _ = msg_sender.send(Command::Redraw);},
                    "m" => { let _ = msg_sender.send(Command::HigherRatio); },
                    "l" => { let _ = msg_sender.send(Command::LowerRatio); },
                    "z" => { let _ = msg_sender.send(Command::Slower); },
                    "x" => { let _ = msg_sender.send(Command::Faster); },
                    _ => ()
                }
            },
            Err(error) => println!("error: {error}")
        }
    }
}

fn generate_board(ratio: f32) -> Board {
    let mut rng = rand::thread_rng();
    let mut new_board = vec!();
    for _ in 0..WIDTH * WIDTH {
        if rng.gen::<f32>() > ratio {
            new_board.push(Cell::Alive(0));
        } else {
            new_board.push(Cell::Dead);
        }
    }

    new_board.try_into().expect("unable to create board array")

}

fn draw_board(state: &Board){
    print!(" ");
    for _ in 0..WIDTH {
        print!("--");
    }
    println!();
    for i in 0..WIDTH {
        print!("|");
        for j in 0..WIDTH {
            print!("{} ", get_cell_display(&state[(i * WIDTH) + j]));
        }
        println!("|");
    }
    print!(" ");
    for _ in 0..WIDTH {
        print!("--");
    }
    println!();
}

fn get_cell_display(cell: &Cell) -> ColoredString {
    match cell {
        Cell::Alive(n) => {
            if *n > 4 {
                "o".blue()
            } else if *n > 3 {
                "o".green()
            } else if *n > 2 {
                "o".yellow()
            } else {
                "o".white()
            }
        },
        Cell::Dead => " ".white()
    }
}

fn get_updated_board(state: &Board) -> Board {
    let mut new_state = vec!();
    for i in 0..state.len() {
        let cell = state[i];
        let neighbours = get_neighbours(i, state);
        let count = count_neighbours(neighbours);
        let new_cell = get_new_cell_state(&cell, count);
        new_state.push(new_cell);
    }

    new_state.try_into().expect("unable to create board array")
}

fn get_new_cell_state(cell: &Cell, neighbour_count: usize) -> Cell {
    match cell {
        Cell::Alive(n) => {
            if neighbour_count > 1 && neighbour_count < 4 {
                Cell::Alive(n + 1)
            } else { 
                Cell:: Dead 
            }
        },
        Cell::Dead => {
            if neighbour_count == 3 {
                Cell::Alive(0)
            } else {
                Cell::Dead
            }
        }
    }
}

fn count_neighbours(neighbours: Neighbours) -> usize {
    let mut count = 0;
    for cell in neighbours {
        match cell {
            Cell::Alive(_) => {
                count += 1;
            },
            _ => ()
        }
    }
    count
}

fn get_neighbours(cell: usize, state: &Board) -> Neighbours {
    let neighbours = [
        get_nw(cell),
        get_n(cell),
        get_ne(cell),
        get_e(cell),
        get_w(cell),
        get_sw(cell),
        get_s(cell),
        get_se(cell)
    ];

    neighbours.iter().map(|address|{
        find_neighbour_state(*address, state)
    }).collect::<Vec<Cell>>().try_into().expect("unable to create array")
}

fn find_neighbour_state(address: Option<usize>, state: &Board) -> Cell {
    match address {
        Some(n) => state[n],
        None => Cell::Dead
    }
}

fn get_nw(cell:usize) -> Option<usize> {
    if cell % WIDTH == 0 || cell < WIDTH {
        None
    } else {
        Some(cell - (WIDTH + 1))
    }
}

fn get_n(cell:usize) -> Option<usize> {
    if cell < WIDTH {
        None
    } else {
        Some(cell - WIDTH)
    }
}

fn get_ne(cell: usize) -> Option<usize> {
    if cell < WIDTH || cell % WIDTH == WIDTH - 1 {
        None
    } else {
        Some(cell - (WIDTH -1 ))
    }
}

fn get_w(cell:usize) -> Option<usize> {
    if cell % WIDTH == 0 {
        None
    } else {
        Some(cell - 1)
    }
}

fn get_e(cell:usize) -> Option<usize> {
    if cell % WIDTH == (WIDTH - 1) {
        None
    } else {
        Some(cell + 1)
    }
}

fn get_sw(cell:usize) -> Option<usize> {
    if cell % WIDTH == 0 || cell >= WIDTH * (WIDTH - 1)  {
        None
    } else {
        Some(cell + (WIDTH - 1))
    }
}

fn get_s(cell:usize) -> Option<usize> {
    if cell >= WIDTH * (WIDTH - 1)  {
        None
    } else {
        Some(cell + WIDTH)
    }
}

fn get_se(cell:usize) -> Option<usize> {
    if cell % WIDTH == WIDTH - 1 || cell >= WIDTH * (WIDTH - 1)  {
        None
    } else {
        Some(cell + WIDTH + 1)
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use Cell::*;

    fn get_board() -> Board {
        [
            Alive(0), Dead, Alive(0), Alive(0), Dead, Alive(0), Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Alive(0), Dead, Alive(0), Alive(0), Dead, Alive(0), Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Alive(0), Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead, Dead, Alive(0), Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead, Dead, Alive(0), Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead, Dead, Alive(0), Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Alive(0), Dead, Alive(0), Alive(0), Dead, Alive(0), Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Alive(0), Dead, Alive(0), Alive(0), Dead, Alive(0), Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Alive(0), Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead, Dead, Alive(0), Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead, Dead, Alive(0), Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead, Dead, Alive(0), Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead
        ]
    }

    #[test]
    fn test_nw(){
        assert_eq!(get_nw(0), None);
        assert_eq!(get_nw(1), None);
        assert_eq!(get_nw(WIDTH + 1), Some(0));
        assert_eq!(get_nw(WIDTH + 2), Some(1));
        assert_eq!(get_nw(WIDTH * 2 + 1), Some(WIDTH));
    }

    #[test]
    fn test_n(){
        assert_eq!(get_n(0), None);
        assert_eq!(get_n(1), None);
        assert_eq!(get_n(WIDTH), Some(0));
    }

    #[test]
    fn test_ne(){
        assert_eq!(get_ne(0), None);
        assert_eq!(get_ne(1), None);
        assert_eq!(get_ne(WIDTH), Some(1));
        assert_eq!(get_ne(2 * WIDTH - 1), None);
    }

    #[test]
    fn test_w(){
        assert_eq!(get_w(0), None);
        assert_eq!(get_w(1), Some(0));
        assert_eq!(get_w(WIDTH), None);
        assert_eq!(get_w(WIDTH - 1), Some(WIDTH - 2));
    }

    #[test]
    fn test_e(){
        assert_eq!(get_e(0), Some(1));
        assert_eq!(get_e(1), Some(2));
        assert_eq!(get_e(WIDTH - 1), None);
        assert_eq!(get_e(WIDTH), Some(WIDTH + 1));
        assert_eq!(get_e(WIDTH * WIDTH - 1), None);
    }

    #[test]
    fn test_sw(){
        assert_eq!(get_sw(0), None);
        assert_eq!(get_sw(1), Some(WIDTH));
        assert_eq!(get_sw(WIDTH + 1), Some(2 * WIDTH));
        assert_eq!(get_sw(WIDTH + 2), Some(2 * WIDTH + 1));
        assert_eq!(get_sw(WIDTH * (WIDTH - 1) + 1), None);
        
    }

    #[test]
    fn test_s(){
        assert_eq!(get_s(0), Some(WIDTH));
        assert_eq!(get_s(1), Some(WIDTH + 1));
        assert_eq!(get_s(WIDTH * (WIDTH - 1) + 1), None);
        assert_eq!(get_s(WIDTH * (WIDTH - 1)), None)
        
    }

    
    #[test]
    fn test_se(){
        assert_eq!(get_se(0), Some(WIDTH + 1));
        assert_eq!(get_se(1), Some(WIDTH + 2));
        assert_eq!(get_se(WIDTH * (WIDTH - 1) + 1), None);
        assert_eq!(get_se((WIDTH * WIDTH) - 1), None);
        
    }

    #[test]
    fn test_neighbour_state_when_none() {
        let board = get_board();
        assert_eq!(find_neighbour_state(None, &board), Dead);
    }

    #[test]
    fn test_neighbour_state_when_alive() {
        let board = get_board();
        assert_eq!(find_neighbour_state(Some(0), &board), Alive(0));
    }

    #[test]
    fn test_neighbour_state_when_dead() {
        let board = get_board();
        assert_eq!(find_neighbour_state(Some(1), &board), Dead);
    }

    #[test]
    fn test_get_neighbours(){
        let board = get_board();
        assert_eq!(get_neighbours(0, &board), [Dead, Dead, Dead, Dead, Dead, Dead, Alive(0), Dead]);
        assert_eq!(get_neighbours(1, &board), [Dead, Dead, Dead, Alive(0), Alive(0), Alive(0), Dead, Alive(0)]);
    }

    #[test]
    fn count_neighbours_returns_alive_count(){
        let neighbours = [Dead, Dead, Alive(0), Dead, Dead, Dead, Dead, Dead];
        assert_eq!(count_neighbours(neighbours), 1);

        let neighbours = [Alive(0), Alive(0), Alive(0), Alive(0), Alive(0), Alive(0), Alive(0), Alive(0)];
        assert_eq!(count_neighbours(neighbours), 8);

        let neighbours = [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead];
        assert_eq!(count_neighbours(neighbours), 0);
    }

    #[test]
    fn when_alive_should_die_for_less_than_2_neighbours(){
        assert_eq!(get_new_cell_state(&Cell::Alive(0), 1), Cell::Dead);
        assert_eq!(get_new_cell_state(&Cell::Alive(0), 0), Cell::Dead);
    }

    
    #[test]
    fn when_alive_should_die_for_more_than_3_neighbours(){
        assert_eq!(get_new_cell_state(&Cell::Alive(0), 4), Cell::Dead);
        assert_eq!(get_new_cell_state(&Cell::Alive(0), 5), Cell::Dead);
    }

    #[test]
    fn when_alive_should_survive_at_2_or_3_neighbours(){
        assert_eq!(get_new_cell_state(&Cell::Alive(0), 2), Cell::Alive(1));
        assert_eq!(get_new_cell_state(&Cell::Alive(0), 3), Cell::Alive(1));
    }

    #[test]
    fn when_a_cell_survives_its_age_increments(){
        assert_eq!(get_new_cell_state(&Cell::Alive(0), 2), Cell::Alive(1));
        assert_eq!(get_new_cell_state(&Cell::Alive(1), 3), Cell::Alive(2));
    }

    #[test]
    fn when_dead_should_remain_dead_when_not_3_neighbours(){
        assert_eq!(get_new_cell_state(&Cell::Dead, 0), Cell::Dead);
        assert_eq!(get_new_cell_state(&Cell::Dead, 1), Cell::Dead);
        assert_eq!(get_new_cell_state(&Cell::Dead, 2), Cell::Dead);
        assert_eq!(get_new_cell_state(&Cell::Dead, 4), Cell::Dead);
        assert_eq!(get_new_cell_state(&Cell::Dead, 5), Cell::Dead);
    }

    #[test]
    fn when_dead_should_alive_for_3_neighbours(){
        assert_eq!(get_new_cell_state(&Cell::Dead, 3), Cell::Alive(0));
    }
}