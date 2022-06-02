use colored::*;
use rand::seq::SliceRandom; // 0.7.2

fn main() {
    let random:[colored::ColoredString; 8] = ["█".green(), "█".blue(), "█".black(),
              "█".cyan(), "█".magenta(),"█".red(),
              "█".white(), "█".yellow()
            ];
    loop {
        print!("{}", random.choose(&mut rand::thread_rng()).expect("msg"));
    }
}
