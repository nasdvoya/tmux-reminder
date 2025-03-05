use std::{
    env, fs,
    process::Command,
    thread,
    time::{Duration, SystemTime},
};

fn get_file_age(file: &String) -> Result<(), Box<dyn std::error::Error>> {
    let meta = fs::metadata(file)?;
    let changed = meta.modified()?;
    let sys_time = SystemTime::now().duration_since(changed)?;

    println!("{:?}", sys_time);

    Ok(())
}

fn main() {
    let home = env::var("HOME").expect("Coulld not get home directory");
    let file_path = format!("{home}/Github/nasdvoya/tmux-reminder/flake.nix");

    get_file_age(&file_path);

    let content = fs::read_to_string(&file_path).unwrap_or_else(|e| {
        eprintln!("Failed to read file: {}", e);
        String::new()
    });

    dbg!(&content);

    let line_content: Vec<String> = content
        .lines()
        .skip(10) // Skip the header
        .filter(|line| !line.trim().is_empty())
        .map(String::from)
        .collect();

    let output = Command::new("tmux")
        .arg("show-option")
        .arg("-v")
        .arg("status")
        .output()
        .expect("Failed to execute tmux");

    let _result = String::from_utf8(output.stdout)
        .expect("Failed to parse output")
        .trim()
        .to_string();

    thread::spawn(move || {
        loop {
            fs::metadata(&file_path);
            for reminder in &line_content {
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

    // println!("{:?}",result);

    // if result == "2" {
    //     _ = Command::new("tmux")
    //         .arg("set-option")
    //         .arg("status")
    //         .arg("on")
    //         .output();
    // } else {
    //     _ = Command::new("tmux")
    //         .arg("set-option")
    //         .arg("status")
    //         .arg("2")
    //         .output();
    // }
}

// Center
// set -g status-right "#[align=absolute-centre] Hello, world! #[align=right]"
// set -ga status-right "#{?window_bigger,[#{window_offset_x}#,#{window_offset_y}] ,}\ 📅 %d.%m.%y 🕰  %H:%M 💻 #{client_user}@#H "
// set -g status-right-length 65
