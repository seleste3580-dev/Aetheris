use clap::Parser;
use colored::*;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::timeout;

#[derive(Parser, Debug)]
#[command(name = "Aetheris")]
#[command(author = "Seleste Technologies")]
#[command(version = "1.0.0")]
#[command(about = "High-Speed Attack Surface Mapper Engine", long_about = None)]
struct Args {
    /// Target IP address (e.g., 192.168.1.1)
    target: String,

    /// Ports to scan (e.g., 80,443, 1-65535)
    #[arg(short, long, default_value = "1-65535")]
    ports: String,

    /// Maximum number of concurrent connections
    #[arg(short, long, default_value_t = 5000)]
    concurrency: usize,

    /// Connection timeout in milliseconds
    #[arg(long, default_value_t = 1500)]
    timeout_ms: u64,

    /// Output file for JSON results
    #[arg(short, long)]
    output: Option<String>,
}

#[derive(Debug, Serialize)]
struct ScanResult {
    ip: String,
    open_ports: Vec<u16>,
    scan_time_ms: u128,
}

fn parse_ports(port_str: &str) -> Vec<u16> {
    let mut ports = Vec::new();
    for part in port_str.split(',') {
        if part.contains('-') {
            let range: Vec<&str> = part.split('-').collect();
            if range.len() == 2 {
                if let (Ok(start), Ok(end)) = (range[0].parse::<u16>(), range[1].parse::<u16>()) {
                    for p in start..=end {
                        ports.push(p);
                    }
                }
            }
        } else if let Ok(port) = part.parse::<u16>() {
            ports.push(port);
        }
    }
    ports
}

async fn scan_port(ip: IpAddr, port: u16, timeout_duration: Duration) -> Option<u16> {
    let socket = SocketAddr::new(ip, port);
    
    // We use a timeout so it doesn't hang indefinitely on filtered/dropped packets
    match timeout(timeout_duration, TcpStream::connect(&socket)).await {
        Ok(Ok(_)) => Some(port), // Connection succeeded
        _ => None, // Connection failed or timed out
    }
}

#[tokio::main]
async fn main() {
    println!("{}", r#"
     _         _   _               _     
    / \   ___ | |_| |__   ___ _ __(_)___ 
   / _ \ / _ \| __| '_ \ / _ \ '__| / __|
  / ___ \  __/| |_| | | |  __/ |  | \__ \
 /_/   \_\___| \__|_| |_|\___|_|  |_|___/
                                         
    "#.cyan().bold());
    println!("{} {}\n", "Aetheris".green().bold(), "v1.0.0 - Enterprise Scanner Engine".white());

    let args = Args::parse();

    let target_ip: IpAddr = match args.target.parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("{} Invalid IP address: {}", "[-]".red().bold(), args.target);
            return;
        }
    };

    let ports = parse_ports(&args.ports);
    if ports.is_empty() {
        eprintln!("{} No valid ports specified.", "[-]".red().bold());
        return;
    }

    println!("{} Target: {}", "[*]".blue().bold(), target_ip);
    println!("{} Ports: {} (total: {})", "[*]".blue().bold(), args.ports, ports.len());
    println!("{} Concurrency: {}", "[*]".blue().bold(), args.concurrency);
    println!("");

    let pb = ProgressBar::new(ports.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ports ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let start_time = Instant::now();
    let timeout_duration = Duration::from_millis(args.timeout_ms);

    // Create a stream of async tasks
    let mut open_ports = stream::iter(ports)
        .map(|port| {
            let pb_clone = pb.clone();
            async move {
                let result = scan_port(target_ip, port, timeout_duration).await;
                pb_clone.inc(1);
                result
            }
        })
        .buffer_unordered(args.concurrency)
        .filter_map(|x| async move { x })
        .collect::<Vec<u16>>()
        .await;

    pb.finish_with_message("Scan complete");
    let scan_duration = start_time.elapsed();

    open_ports.sort_unstable();

    println!("\n{} Scan completed in {:.2}s", "[+]".green().bold(), scan_duration.as_secs_f64());
    println!("{} Open Ports:", "[+]".green().bold());
    
    if open_ports.is_empty() {
        println!("    No open ports found.");
    } else {
        for port in &open_ports {
            println!("    - {} {}", port.to_string().yellow().bold(), "open".green());
        }
    }

    if let Some(output_file) = args.output {
        let result = ScanResult {
            ip: target_ip.to_string(),
            open_ports,
            scan_time_ms: scan_duration.as_millis(),
        };

        if let Ok(json) = serde_json::to_string_pretty(&result) {
            if let Ok(mut file) = File::create(&output_file) {
                if file.write_all(json.as_bytes()).is_ok() {
                    println!("\n{} Results saved to {}", "[+]".green().bold(), output_file.white().bold());
                } else {
                    eprintln!("\n{} Failed to write to {}", "[-]".red().bold(), output_file);
                }
            } else {
                eprintln!("\n{} Failed to create file {}", "[-]".red().bold(), output_file);
            }
        }
    }
}
