use std::{
    env::{self, args},
    fs,
    process::Command,
    str::FromStr,
    thread,
    time::{Duration, SystemTime},
};

fn get_file_age(file: &String) -> Result<(), Box<dyn std::error::Error>> {
    let meta = fs::metadata(file)?;
    let changed = meta.modified()?;
    let sys_time = SystemTime::now().duration_since(changed)?;
    Ok(())
}

struct Configuration {
    file_path: String,
    update_interval: u16,
}

fn setup() -> Result<Configuration, String> {
    let interval = Command::new("tmux")
        .arg("show-option")
        .arg("-gv")
        .arg("@tmux_reminder_interval")
        .output()
        .expect("Failed to get tmux option");

    let file_path = Command::new("tmux")
        .arg("show-option")
        .arg("-gv")
        .arg("@tmux_reminder_file")
        .output()
        .expect("Failepub d to get tmux option");

    let setup_interval = String::from_utf8(interval.stdout)
        .expect("")
        .trim()
        .parse::<u16>()
        .expect("Failed to parse interval");

    let setup_path = String::from_utf8(file_path.stdout).expect("s");

    Ok(Configuration {
        file_path: setup_path,
        update_interval: setup_interval,
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let home = env::var("HOME").expect("Coulld not get home directory");
    let file_path = format!("{home}/Github/nasdvoya/tmux-reminder/flake.nix");

    // get_file_age(&file_path);

    let content = fs::read_to_string(&file_path).unwrap_or_else(|e| {
        eprintln!("Failed to read file: {}", e);
        String::new()
    });

    dbg!(&content);

    let line_content: Vec<String> = content
        .lines()
        .skip(2) // Skip the header
        .filter(|line| !line.trim().is_empty())
        .map(String::from)
        .collect();

    thread::spawn(move || {
        loop {
            for reminder in &line_content {
                println!("{}", reminder);
                _ = Command::new("tmux")
                    .arg("set")
                    .arg("-g")
                    .arg("status-right")
                    .arg(format!(
                        "#[align=absolute-centre] {} #[align=right]",
                        reminder
                    ))
                    .output();
                thread::sleep(Duration::from_secs(2));
            }
        }
    });
}

// How Your Rust Code Should Handle These Options
//
// Your Rust program should retrieve these values using:
//

// Center
// set -g status-right "#[align=absolute-centre] Hello, world! #[align=right]"
// set -ga status-right "#{?window_bigger,[#{window_offset_x}#,#{window_offset_y}] ,}\ 📅 %d.%m.%y 🕰  %H:%M 💻 #{client_user}@#H "
// set -g status-right-length 65
