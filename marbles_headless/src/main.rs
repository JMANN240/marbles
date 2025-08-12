use clap::Parser;
use dotenvy::dotenv;
use lib::{scenes::scene_3, simulation::Simulation, Config};
use rand::rng;
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
pub struct Cli {
    #[arg(long, default_value_t = 1)]
    renders: usize,

    #[arg(short, long)]
    instagram: bool,

    #[arg(short, long)]
    youtube: bool,

    #[arg(long, default_value_t = 3)]
    countdown_seconds: usize,

    #[arg(long, default_value_t = 10)]
    reset_seconds: usize,

    #[arg(long, default_value_t = 1.0)]
    timescale: f64,

    #[arg(long, default_value_t = 100)]
    physics_steps: usize,

    #[arg(long, default_value_t = 0)]
    race_offset: usize,
}

fn main() {
    dotenv().unwrap();
    tracing::subscriber::set_global_default(FmtSubscriber::default()).unwrap();
    let mut rng = rng();
    let cli = Cli::parse();

    let config_string = std::fs::read_to_string("config.toml").unwrap();
    let config = toml::from_str::<Config>(&config_string).unwrap();

    // let simulation: Simulation = Simulation::new(scene_3(config.get_balls().clone()), cli.countdown_seconds as f64, cli.reset_seconds as f64, "TEST".to_string());
}
