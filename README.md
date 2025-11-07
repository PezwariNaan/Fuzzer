## Rust HTTP Fuzzer

This is a simple asynchronous HTTP fuzzer written in Rust. It reads a wordlist from a file and sends GET requests to a target base URL, appending each word from the list as a path. Requests are executed concurrently to improve performance.

### Features
- Reads a wordlist file (`one word per line`) and trims whitespace.
- Reuses a single `reqwest::Client` for efficient connection pooling.
- Sends concurrent HTTP requests using async streams and buffered concurrency.
- Prints the HTTP status code for each request.
- Easy to configure base URL, wordlist path, and concurrency level.

### Example Usage
```
fuzzer --url https://example.com --wordlist wordlist.txt

Loaded 5 words
admin -> 301 Moved Permanently
login -> 200 OK
dashboard -> 403 Forbidden
test.php -> 404 Not Found
hidden/ -> 200 OK
```

### TODO
- Add recursive calls to found URLs
- Add subdomain enumeration

