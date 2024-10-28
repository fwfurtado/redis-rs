use redis::server;

fn main() {
    server::listen("localhost", 6379);
}