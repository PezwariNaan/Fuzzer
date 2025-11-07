use reqwest::header::{USER_AGENT};
use reqwest::{Client, StatusCode};
use std::{fs, io, time::Duration, error::Error};
use futures::stream::{self, StreamExt};
use clap::Parser;

struct Fuzzer {
    client   : Client, 
    word_list: Vec<String>,
    base_url : String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = "Simple Fuzzer Writen In Rust.")]
struct Args {
    #[arg(short, long)]
    wordlist: String,

    #[arg(short, long)]
    url: String,
}

impl Fuzzer {
    pub fn new(base_url: &str) -> Result<Self, Box<dyn Error>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()?;

        Ok(Self {
            client, 
            word_list: Vec::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
        })
    }

    pub fn load_wordlist(&mut self, file_path: &str) -> Result<(), io::Error> { 
        let contents = fs::read_to_string(file_path)?;
        let lines = contents
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        self.word_list = lines;

        Ok(())
    }

    pub async fn make_request(&self, path: &str) -> Result<StatusCode, reqwest::Error> {
        let base = &self.base_url;
        let url = if path.starts_with('/') {
            format!("{}{}", base, path)
        } else {
            format!("{}/{}", base, path)
        };

        let response = self
            .client
            .get(&url)
            .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:144.0) Gecko/20100101 Firefox/144.0")
            .send()
            .await?;

        Ok(response.status())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut fuzzer = Fuzzer::new(&args.url)?;

    fuzzer.load_wordlist(&args.wordlist)?;
    println!("Loaded {} words", fuzzer.word_list.len());

    let concurrency: usize = 20;
    
    let paths = fuzzer.word_list
        .iter()
        .take(50)
        .cloned()
        .collect::<Vec<_>>();

    let fetches = stream::iter(paths.into_iter().map(|path| {
        let fuzzer_ref = &fuzzer;
        async move {
            match fuzzer_ref.make_request(&path).await {
                Ok(status) => println!("{} -> {}", path, status),
                Err(e) => eprintln!("request {} failed: {}", path, e), 
            }
        }
    }))
    .buffer_unordered(concurrency);

    fetches.for_each(|_| async {}).await;
    

    Ok(())
}

