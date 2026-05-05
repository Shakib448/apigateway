use api_gateway::config::VALID_AUTH_TOKEN;

fn main() {
    println!("Admin can: {:?}", VALID_AUTH_TOKEN.get("example-token"));
}
