use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    // whether to loop sound effects
    pub play_in_loop: Option<bool>,

    // how many loops should occur; leave unset for infinite loops
    pub loop_count: Option<u64>,

    // the file paths of the audio files you want to play
    pub file_paths: Vec<String>,

    // shuffle the files or play them in the order they're configured
    pub shuffle: Option<bool>,
}

impl Config {
    pub fn load_config_from_file<R: Read>(file: R) -> Self {
        let mut config: Config = serde_yaml::from_reader(file).unwrap();
        config.play_in_loop = Some(config.play_in_loop.unwrap_or(true));
        config.shuffle = Some(config.shuffle.unwrap_or(false));
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    #[test]
    fn read_full_config_from_file() {
        let raw_yaml = concat!(
            "play_in_loop: true\n",
            "shuffle: true\n",
            "file_paths:\n",
            "  - /cool/path/to/audio1.mp3\n",
            "  - /cool/path/to/audio2.mp3\n",
            "  - /cool/path/to/audio3.mp3\n",
            "  - /cool/path/to/audio4.mp3\n",
        );
        let expected_config = Config {
            play_in_loop: Some(true),
            shuffle: Some(true),
            loop_count: None,
            file_paths: vec![
                "/cool/path/to/audio1.mp3",
                "/cool/path/to/audio2.mp3",
                "/cool/path/to/audio3.mp3",
                "/cool/path/to/audio4.mp3",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        };
        let cursor = Cursor::new(raw_yaml);
        let config = Config::load_config_from_file(cursor);
        assert_eq!(config, expected_config);
    }

    #[test]
    fn read_partial_config_from_file() {
        let raw_yaml = concat!(
            "play_in_loop: true\n",
            "file_paths:\n",
            "  - /cool/path/to/audio1.mp3\n",
            "  - /cool/path/to/audio2.mp3\n",
            "  - /cool/path/to/audio3.mp3\n",
            "  - /cool/path/to/audio4.mp3\n",
        );
        let expected_config = Config {
            play_in_loop: Some(true),
            shuffle: Some(false),
            loop_count: None,
            file_paths: vec![
                "/cool/path/to/audio1.mp3",
                "/cool/path/to/audio2.mp3",
                "/cool/path/to/audio3.mp3",
                "/cool/path/to/audio4.mp3",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        };
        let cursor = Cursor::new(raw_yaml);
        let config = Config::load_config_from_file(cursor);
        assert_eq!(config, expected_config);
    }
}
