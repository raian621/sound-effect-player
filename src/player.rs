use std::{fs::File, io::BufReader};

use rand::rng;
use rand::seq::SliceRandom;
use rodio::{Decoder, OutputStream, Source, source::Buffered};

use crate::cache::LruCache;

pub type AudioSource = Buffered<Decoder<BufReader<File>>>;

pub struct Player {
    audio_sink: rodio::Sink,
    audio_cache: LruCache<String, AudioSource>,
    #[allow(dead_code)] // we need this output stream to stay in scope to continue playing sounds
    output_stream: OutputStream,
}

pub enum Loops {
    None,
    Count(u64),
    Infinite,
}

impl Player {
    pub fn new(audio_cache: LruCache<String, AudioSource>) -> Self {
        let output_stream =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        Self {
            audio_sink: rodio::Sink::connect_new(output_stream.mixer()),
            output_stream,
            audio_cache,
        }
    }

    pub fn start_playing(&mut self, loops: Loops, shuffle: bool, _file_paths: &[String]) {
        match loops {
            Loops::None => self.play_audio_clips(shuffle, _file_paths),
            Loops::Count(_) | Loops::Infinite => {
                self.play_audio_clips_in_loop(loops, shuffle, _file_paths)
            }
        }
    }

    fn play_audio_clips(&mut self, shuffle: bool, file_paths: &[String]) {
        let mut indices: Vec<usize> = (0..file_paths.len()).collect();
        if shuffle {
            let mut rng = rng();
            indices.shuffle(&mut rng);
        }
        indices
            .iter()
            .for_each(|i| self.play_audio_clip(&file_paths[*i]));
    }

    fn play_audio_clip(&mut self, file_path: &String) {
        let source = self.load_audio_source(file_path);
        self.audio_sink.append(source);
        self.audio_sink.play();
        self.audio_sink.sleep_until_end();
    }

    fn play_audio_clips_in_loop(&mut self, loops: Loops, shuffle: bool, file_paths: &[String]) {
        match loops {
            Loops::Infinite => loop {
                self.play_audio_clips(shuffle, file_paths)
            },
            Loops::Count(count) => {
                (0..count as usize).for_each(|_| self.play_audio_clips(shuffle, file_paths))
            }
            _ => panic!("this code should not be reached"),
        };
    }

    fn load_audio_source(&mut self, file_path: &String) -> AudioSource {
        if let Some(source) = self.audio_cache.get(file_path) {
            source.clone()
        } else {
            let source = Decoder::try_from(File::open(file_path).unwrap())
                .unwrap()
                .buffered();
            let byte_size = source.channels() as usize
                * source.sample_rate() as usize
                * source.current_span_len().expect("should have samples");
            self.audio_cache
                .push(file_path.clone(), source.clone(), byte_size);
            source
        }
    }
}
