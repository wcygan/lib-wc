use clap::Parser;

/// A load testing tool
#[derive(Debug, Parser)]
#[command(about = "A tool to load test a server", long_about = None, arg_required_else_help = true)]
pub struct Cli {
    /// The URL to send requests to
    #[arg(short = 'u', long, value_name = "URL", required = true)]
    pub url: String,

    /// The number of connections to use
    #[arg(short = 'c', long, value_name = "CONNECTIONS", default_value_t = 8, value_parser = clap::value_parser ! (u16).range(1..512))]
    pub connections: u16,

    /// The amount of seconds to run the test for. If not specified, the test will run until an interrupt signal is received
    #[arg(short = 't', long, value_name = "TIME", value_parser = clap::value_parser ! (u16).range(1..512))]
    pub time: Option<u16>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let cli = Cli::try_parse_from([
            "load",
            "-u",
            "http://localhost:8080",
            "-c",
            "10",
            "-t",
            "100",
        ])
        .unwrap();
        assert_eq!(cli.url, "http://localhost:8080");
        assert_eq!(cli.connections, 10);
        assert_eq!(cli.time, Some(100));
    }

    #[test]
    fn test_parse_with_invalid_requests() {
        let cli =
            Cli::try_parse_from(["load", "-u", "http://localhost:8080", "-r", "0", "-c", "10"]);
        assert!(cli.is_err());
    }

    #[test]
    fn test_parse_with_invalid_connections() {
        let cli = Cli::try_parse_from([
            "load",
            "-u",
            "http://localhost:8080",
            "-r",
            "1000",
            "-c",
            "0",
        ]);
        assert!(cli.is_err());
    }

    #[test]
    fn test_parse_with_invalid_timeout() {
        let cli = Cli::try_parse_from([
            "load",
            "-u",
            "http://localhost:8080",
            "-r",
            "1000",
            "-c",
            "10",
            "-t",
            "0",
        ]);
        assert!(cli.is_err());
    }
}
