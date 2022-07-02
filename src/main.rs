use std::io;

type Matrix = Vec<Vec<&'static str>>;

struct Game {
    map: Matrix,
    ball_x: u8,
    ball_y: u8,
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get input");
    input
}

fn verify_user_input(input: &String) -> bool {
    if input.chars().next().unwrap().is_ascii_alphabetic() {
        true
    } else {
        false
    }
}

fn change_wall_direction(e: &'static str) -> &'static str{
    match e {
        "|" => "-",
        "-" => "|",
        _ => e,
    }
}

impl Game {
    fn rotate_map(&mut self, rotation: char) {
        let map = &self.map;
        let mut result: Vec<Vec<&str>> = vec![];
        let row = map.len();
        let col = map[0].len();

        for i in 0..col {
            let mut temp: Vec<&str> = vec![];

            for j in 0..row {
                // Clockwise
                if rotation == 'w' {
                    let mut e = map[row-1-j][i];
                    e = change_wall_direction(e);

                    if e == "o" {
                        self.ball_x = j as u8;
                        self.ball_y = i as u8;
                    }

                    temp.push(e);
                }
                // Counter-clockwise
                else if rotation == 'c' {
                    let mut e = map[j][col-1-i];
                    e = change_wall_direction(e);

                    if e == "o" {
                        self.ball_x = j as u8;
                        self.ball_y = i as u8;
                    }

                    temp.push(e);
                }
            }
            result.push(temp);
        }
        self.map = result;
    }

    fn display(&self) {
        for i in self.map.iter() {
            for j in i.iter() {
                print!("{}", j);
            }
            print!("\n");
        }
    }

    fn handle_physics(&mut self) {
        let mut map = &mut self.map;
        let row = map.len() as usize;
        let x = self.ball_x as usize;
        let y = self.ball_y as usize;
        
        for i in y..row {
            if map[i][x] == "-" {
                map[i-1][x] = "o";
                map[y][x] = ".";
                self.map = map.to_vec();
                return ();
            }
        }

        map[row-1][x] = "o";
        map[y][x] = ".";
        self.map = map.to_vec();
        return;
    }
}

fn main() {
    let mut game = Game {
        map: get_map(),
        ball_x: 0,
        ball_y: 4,
    };
    let mut additional_output = "";

    loop {
        println!("{}", additional_output);
        clear();
        game.display();

        let input = get_user_input();
        if verify_user_input(&input) {
            match input.chars().next().unwrap() {
                'a' => game.rotate_map('c'),
                'd' => game.rotate_map('w'),
                _ => continue,
            }

            game.handle_physics();
        } else {
            additional_output = "Invaild input!"
        }
    }
}

fn get_map() -> Matrix {
    // i don't know what to say...
    vec![
        vec!["O", ".", ".", ".", "."],
        vec![".", ".", "|", ".", "."],
        vec!["-", "-", "-", ".", "."],
        vec![".", ".", ".", ".", "."],
        vec!["o", ".", ".", ".", "."],
    ]
}

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
