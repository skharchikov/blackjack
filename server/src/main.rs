use server::Settings;

fn main() {
    let settings = Settings::load_or_die().expect("Failed to load configuration");
    println!("Application settings: {:?}", settings.application);
}
