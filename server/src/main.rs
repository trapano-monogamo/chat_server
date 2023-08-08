mod server;
mod thread_pool;

fn main() {
    if let Err(e) = server::start() {
        println!("[Error]: {e}");
    }
}
