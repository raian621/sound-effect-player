use std::fs::File;

use crate::{
    args::Args,
    cache::LruCache,
    config::Config,
    player::{Loops, Player},
};

pub mod args;
pub mod cache;
pub mod config;
pub mod player;

const MAX_CACHE_BYTE_SIZE: usize = 100_000_000; // 100 MB

fn main() {
    let args = Args::get_args();
    let config_path = args.config_path.unwrap();
    let file = File::open(config_path).unwrap();
    let config = Config::load_config_from_file(file);
    let audio_cache = LruCache::new(MAX_CACHE_BYTE_SIZE);
    println!("{:?}", config.file_paths);
    let loops = if config.play_in_loop.unwrap() {
        match config.loop_count {
            Some(count) => Loops::Count(count),
            None => Loops::Infinite,
        }
    } else {
        Loops::None
    };
    let mut player = Player::new(audio_cache);
    player.start_playing(loops, config.shuffle.unwrap(), &config.file_paths);
    println!("{:?}", config);
}
