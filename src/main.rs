const TIME_180: i32 = 600;
const TIME_RIGHT: i32 = 340;
const TIME_LEFT: i32 = 380;
const TIME_PAUSE: i32 = 400;

fn parse_dir(dir: &str) -> i8 {
    match dir {
        "up" => 0,
        "right" => 1,
        "down" => 2,
        "left" => 3,
        _ => panic!("Invalid direction"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut gen: Vec<[i32; 2]> = vec![];
    let mut curr_dir: i8 = 0;

    // initial pause
    gen.push([1000, 0]);

    for line in std::fs::read_to_string(args[1].clone()).unwrap().lines() {
        let (dir, mag) = line.split_once(' ').unwrap();
        let dir = parse_dir(dir.to_lowercase().as_str());
        let mag = mag.parse::<f32>().unwrap();

        if dir != curr_dir {
            if (dir - curr_dir).abs() == 2 {
                // 180
                gen.push([TIME_180, 2]);
                gen.push([TIME_PAUSE, 0]);
            } else if (dir - curr_dir + 4) % 4 == 1 {
                // right turn
                gen.push([TIME_RIGHT, 2]);
                gen.push([TIME_PAUSE, 0]);
            } else {
                // left turn
                gen.push([TIME_LEFT, 3]);
                gen.push([TIME_PAUSE, 0]);
            }
        }

        // drive forward
        gen.push([(mag * 1000.0) as i32, 1]);
        gen.push([TIME_PAUSE, 0]);

        curr_dir = dir;
    }

    let j = serde_json::to_string(&gen).unwrap();
    std::fs::write("rt.json", j).unwrap();
}
