use std::process::Command;

pub fn get_running_containers() -> Option<Vec<String>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("docker ps --format '{{.Names}}'")
        .output()
        .expect("Failed to get running containers");

    if output.status.success() {
        let containers = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(String::from)
            .collect::<Vec<_>>();

        if containers.is_empty() {
            None
        } else {
            Some(containers)
        }
    } else {
        None
    }
}

pub fn stop_or_kill_containers(containers: &[String], kill: bool) {
    let command = if kill { "kill" } else { "stop" };
    let container_list = containers.join(" ");

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("docker {} {}", command, container_list))
        .output()
        .expect("Failed to execute docker command");

    if output.status.success() {
        println!("(╯°□°）╯︵ ┻━┻\nSelected containers dosukoi!");
    } else {
        eprintln!(
            "Error stopping containers: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
