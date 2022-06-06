use std::env;

fn main() {
    let mut url = "https://cheat.sh/".to_string();
    let query = String::new();
    let args: Vec<_> = env::args().collect();
    let length = args.len();
    if length >= 1 {
        url.push_str(format!("{}/", args[1]).as_str());
        for i in 2..length {
            if i == length - 1 {
                url.push_str(format!("{}", args[i]).as_str());
            } else {
                url.push_str(format!("{}+", args[i]).as_str());
            }
        }
    }
    //concat query to &str url
    url.push_str(&query);
    let child = std::process::Command::new("curl")
        .arg(url)
        .spawn()
        .expect("failed to execute process");
    let output = child.wait_with_output().expect("failed to wait on child");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
