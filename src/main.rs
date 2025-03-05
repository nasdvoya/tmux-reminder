use std::{
    fs,
    process::Command,
    thread,
    time::{Duration, SystemTime},
};

fn main() {
    let file = "/Github/heyo/notes/todo.md";

    let metadata = fs::metadata(&file).expect("Failed to get metadata");
    let changed = metadata.modified().expect("when i hit the music");
    std::time::Duration::from_secs(1);

    let sys_time = SystemTime::now().duration_since(changed);
    println!("Squabble up {:?}", sys_time);
    // println!("{:?}", file_type);
    // Ok(Metadata { file_type: FileType { is_file: true, is_dir: false, is_symlink: false, .. }, permissions: Permissions(FilePermissions { mode: 0o100664 (-rw-rw-r--) }), len: 629, modified: SystemTime { tv_sec: 1741108391, tv_nsec: 570236412 }, accessed: SystemTime { tv_sec: 1741108391, tv_nsec: 570236412 }, created: SystemTime { tv_sec: 1741108391, tv_nsec: 570236412 }, .. })

    let content = fs::read_to_string(&file).expect("crazy spooky");

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

    let result = String::from_utf8(output.stdout)
        .expect("Failed to parse output")
        .trim()
        .to_string();

    thread::spawn(move || {
        loop {
            fs::metadata(&file);
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
