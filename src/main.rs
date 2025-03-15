use std::{
    env::{self},
    fs::{self, File},
    path::PathBuf,
    process::Command,
};

#[derive(Debug)]
struct Configuration {
    reminder_file: String,
    state_path: PathBuf,
    update_interval: usize,
}

impl Configuration {
    fn new() -> Self {
        Self {
            reminder_file: Self::get_reminder_file(),
            update_interval: Self::get_reminder_interval(),
            state_path: Self::get_state_path(),
        }
    }
    fn get_state_path() -> PathBuf {
        let home = env::var("HOME").expect("Could not get home directory");
        let config_dir = PathBuf::from(home).join("Github").join("tmux-reminder");

        let _ = fs::create_dir(&config_dir);
        config_dir.join("state.txt")
    }

    fn get_reminder_interval() -> usize {
        let default_interval = "5".to_string();
        let interval = Command::new("tmux")
            .arg("show-option")
            .arg("-gv")
            .arg("@tmux_reminder_interval")
            .output()
            .map_err(|e| format!("Failed to execute tmux command for interval update: {}", e))
            .ok();

        let setup_interval = interval
            .and_then(|out| String::from_utf8(out.stdout).ok())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or(default_interval)
            .parse::<usize>()
            .unwrap_or(5);

        setup_interval
    }

    fn get_reminder_file() -> String {
        let default_path = format!(
            "{}/Github/nasdvoya/tmux-reminder/flake.nix",
            env::var("HOME").unwrap_or_else(|_| "/tmp".to_string())
        );

        let output = Command::new("tmux")
            .arg("show-option")
            .arg("-gv")
            .arg("@tmux_reminder_file")
            .output()
            .map_err(|e| eprintln!("Failed to execute tmux command for interval update: {}", e))
            .ok();

        let path = output
            .and_then(|out| String::from_utf8(out.stdout).ok())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or(default_path);

        path
    }

    fn get_reminder_index(&self) -> usize {
        let note_index = match File::create_new(&self.state_path) {
            Ok(_) => 0,
            Err(e) => {
                let content =
                    fs::read_to_string(&self.state_path).expect("Failed to read state file");

                content
                    .lines()
                    .next()
                    .and_then(|s| s.trim().parse::<usize>().ok())
                    .unwrap_or(0)
            }
        };
        note_index
    }

    fn get_file_content(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&self.reminder_file)?;

        let line_content: Vec<String> = content
            .lines()
            .skip(2)
            .filter(|line| !line.trim().is_empty())
            .map(String::from)
            .collect();

        return Ok(line_content);
    }
}

fn main() {
    let plugin_configuration = Configuration::new();
    let mut reminder_index = plugin_configuration.get_reminder_index();
    let all_reminders = match plugin_configuration.get_file_content() {
        Ok(content) => content,
        Err(e) => {
            vec!["No reminders found.".to_string()]
        }
    };

    let reminder = if reminder_index < all_reminders.len() {
        &all_reminders[reminder_index]
    } else {
        fs::write("state.txt", "0").expect("Failed to reset state file");
        reminder_index = 0;
        &all_reminders[0]
    };

    println!("#[align=absolute-centre] {} #[align=right]", reminder);

    reminder_index = (reminder_index + 1) % all_reminders.len();
    fs::write(&plugin_configuration.state_path, reminder_index.to_string())
        .expect("Failed to reset state file");
}
