mod crawler;

const STARTING_URL: &str = "https://www.lemonde.fr/";
const N_LOOPS: u16 = 1000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crawler::run(STARTING_URL, N_LOOPS)?;
    Ok(())
}
