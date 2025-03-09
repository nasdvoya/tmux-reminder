use std::{
    env::{self},
    fs,
    process::Command,
    thread,
    time::{Duration, SystemTime},
};

#[derive(Debug)]
struct Configuration {
    file_path: String,
    update_interval: u64,
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
        .expect("Failed to get tmux option");

    let setup_interval = String::from_utf8(interval.stdout)
        .expect("")
        .trim()
        .parse::<u64>()
        .expect("Failed to parse interval");
    let setup_path = String::from_utf8(file_path.stdout)
        .expect("s")
        .trim()
        .to_string();

    Ok(Configuration {
        file_path: setup_path,
        update_interval: setup_interval,
    })
}

fn get_file_age(file: &String) -> Result<Duration, Box<dyn std::error::Error>> {
    let meta = fs::metadata(file)?.modified()?;
    let time = SystemTime::now().duration_since(meta)?;
    Ok(time)
}

fn get_file_content(file_path: &String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(&file_path)?;
    dbg!(&content);

    let line_content: Vec<String> = content
        .lines()
        .skip(2)
        .filter(|line| !line.trim().is_empty())
        .map(String::from)
        .collect();

    return Ok(line_content);
}

fn main() {
    let plugin_configuration = match setup() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error during plugin setup, will use defaults, error: {}", e);
            let home = env::var("HOME").expect("Coulld not get home directory");
            Configuration {
                file_path: format!("{home}/Github/nasdvoya/tmux-reminder/flake.nix"),
                update_interval: 5,
            }
        }
    };
    let notes_content = match get_file_age(&plugin_configuration.file_path) {
        Ok(file_age) if file_age >= Duration::from_secs(plugin_configuration.update_interval) => {
            match get_file_content(&plugin_configuration.file_path) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading file content: {}", e);
                    vec!["".to_string()]
                }
            }
        }
        Ok(_) => {
            vec!["".to_string()]
        }
        Err(e) => {
            dbg!(&plugin_configuration);
            eprintln!("General error reading file content: {}", e);
            vec!["".to_string()]
        }
    };

    for reminder in &notes_content {
        println!("#[align=absolute-centre] {} #[align=right]", reminder);
    }
}

// Center
// set -g status-right "#[align=absolute-centre] Hello, world! #[align=right]"
// set -ga status-right "#{?window_bigger,[#{window_offset_x}#,#{window_offset_y}] ,}\ 📅 %d.%m.%y 🕰  %H:%M 💻 #{client_user}@#H "
// set -g status-right-length 65
