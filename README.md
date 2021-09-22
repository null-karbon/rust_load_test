# rust_load_test
Rust load test 0.1.0  
Nick K. <ngkdev@gmail.com>  
Runs a basic load test of repeated GET requests to a url.

## USAGE:
    basic-load-tester.exe [FLAGS] [OPTIONS] --url <url>

### FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Run in verbose mode

### OPTIONS:
    -u, --url <url>          **Required** Url to test
    -d, --delay <delay>      Delay between requests in millis
    -n, --number <number>    Number of requests to make
