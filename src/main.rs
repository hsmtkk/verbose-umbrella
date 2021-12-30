mod line;

fn main(){
    let channel_access_token = std::env::var("CHANNEL_ACCESS_TOKEN").expect("CHANNEL_ACCESS_TOKEN environment variable must be defined");
    let client = line::Client::new(&channel_access_token);
    let mut msg = String::new();
    for (key, value) in std::env::vars() {
        if key.starts_with("CI_COMMIT_") {
            let line = format!("{} {}", key, value);
            msg += &line;
            msg += "\n";
        }
    }
    client.broadcast(vec![&msg]).expect("broadcast");
}