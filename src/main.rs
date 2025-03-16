use std::{
    env::{self},
    fs::{self, File},
    io,
    path::PathBuf,
    process::{self, Command},
};

#[derive(Debug)]
struct Configuration {
    reminder_file: String,
    state_path: PathBuf,
}

fn get_reminder_file() -> Result<String, io::Error> {
    let output = Command::new("tmux")
        .arg("show-option")
        .arg("-gv")
        .arg("@tmux_reminder_file")
        .output()?;

    let path = String::from_utf8(output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        .trim()
        .to_string();

    if path.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Reminder path is empty",
        ));
    }

    Ok(path)
}

fn get_state_path() -> Result<PathBuf, io::Error> {
    let current_dir = env::current_dir()?;
    Ok(current_dir.join("state.txt"))
}

impl Configuration {
    fn new() -> Result<Self, io::Error> {
        Ok(Self {
            reminder_file: get_reminder_file()?,
            state_path: get_state_path()?,
        })
    }

    fn get_reminder_index(&self) -> usize {
        // Try to create file, if it exists, return 0 as index
        let note_index = match File::create_new(&self.state_path) {
            Ok(_) => 0,
            Err(_) => {
                let content = fs::read_to_string(&self.state_path).unwrap_or("0".to_string());
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
            .skip(10)
            .filter(|line| !line.trim().is_empty())
            .map(String::from)
            .collect();

        return Ok(line_content);
    }
}

fn main() {
    let plugin_configuration = match Configuration::new() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to initiliaze plugin {}", e);
            process::exit(1);
        }
    };

    let mut reminder_index = plugin_configuration.get_reminder_index();
    let all_reminders = match plugin_configuration.get_file_content() {
        Ok(content) => content,
        Err(_) => {
            vec!["No reminders found.".to_string()]
        }
    };

    let reminder = if reminder_index < all_reminders.len() {
        &all_reminders[reminder_index]
    } else {
        &all_reminders[reminder_index]
    };

    println!("#[align=absolute-centre] {} #[align=right]", reminder);

    reminder_index = (reminder_index + 1) % all_reminders.len();
    fs::write(&plugin_configuration.state_path, reminder_index.to_string())
        .expect("Failed to reset state file");
}
