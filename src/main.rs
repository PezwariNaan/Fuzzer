use reqwest::header::{USER_AGENT};
use reqwest::{Client, StatusCode};
use std::{fs, io, time::Duration, error::Error};
use futures::stream::{self, StreamExt};

struct Fuzzer {
    client   : Client, 
    word_list: Vec<String>,
    base_url : String,
}

impl Fuzzer {
    pub fn new(base_url: &str) -> Result<Self, Box<dyn Error>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(2))
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
            .header(USER_AGENT, "hello-there")
            .send()
            .await?;

        Ok(response.status())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut fuzzer = Fuzzer::new("https://httpbin.org")?;

    fuzzer.load_wordlist("test_wordlist.txt")?;
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

