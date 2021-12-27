extern crate clap;

use std::path::Path;
use std::process;
use std::process::{Command, Stdio};

use clap::{App, Arg};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WardArgs {
    pub target: String,
    pub stdin: bool,
    pub verify: String,
    pub file: String,
    pub update_fingerprint: bool,
    pub csv: String,
    pub json: String,
    pub proxy: String,
    pub timeout: u64,
    pub plugins: String,
    pub update_plugins: bool,
    pub update_self: bool,
    pub thread: u32,
    pub ip: String,
    pub uuid: String,
}

impl WardArgs {
    pub fn new() -> Self {
        let app = App::new("ObserverWard")
            .version("0.0.1")
            // .about("about: Community based web fingerprint analysis tool.")
            .author("author: Kali-Team")
            .arg(
                Arg::with_name("target")
                    .short("t")
                    .long("target")
                    .value_name("TARGET")
                    .help("The target URL(s) (required, unless --stdin used)"),
            )
            .arg(
                Arg::with_name("ip")
                    .long("ip")
                    .value_name("IP")
                    .help("The target ip (ex: [127.0.0.1|192.168.1.0/24])"),
            )
            .arg(
                Arg::with_name("server")
                    .short("s")
                    .long("server")
                    .value_name("SERVER")
                    .help("Start a web API service (127.0.0.1:8080)"),
            )
            .arg(
                Arg::with_name("stdin")
                    .long("stdin")
                    .takes_value(false)
                    .help("Read url(s) from STDIN")
                    .conflicts_with("url"),
            )
            .arg(
                Arg::with_name("file")
                    .short("f")
                    .long("file")
                    .value_name("FILE")
                    .help("Read the target from the file"),
            )
            .arg(
                Arg::with_name("csv")
                    .short("c")
                    .long("csv")
                    .value_name("CSV")
                    .help("Export to the csv file or Import form the csv file"),
            )
            .arg(
                Arg::with_name("json")
                    .short("j")
                    .long("json")
                    .value_name("JSON")
                    .help("Export to the json file or Import form the json file"),
            )
            .arg(
                Arg::with_name("proxy")
                    .long("proxy")
                    .takes_value(true)
                    .value_name("PROXY")
                    .help("Proxy to use for requests (ex: [http(s)|socks5(h)]://host:port)"),
            )
            .arg(
                Arg::with_name("timeout")
                    .long("timeout")
                    .takes_value(true)
                    .default_value("10")
                    .value_name("TIMEOUT")
                    .help("Set request timeout."),
            )
            .arg(
                Arg::with_name("thread")
                    .long("thread")
                    .takes_value(true)
                    .default_value("100")
                    .value_name("THREAD")
                    .help("Number of concurrent threads."),
            )
            .arg(
                Arg::with_name("verify")
                    .long("verify")
                    .takes_value(true)
                    .help("Validate the specified yaml file"),
            )
            .arg(
                Arg::with_name("uuid")
                    .long("uuid")
                    .takes_value(true)
                    .help("UUID"),
            )
            .arg(
                Arg::with_name("plugins")
                    .long("plugins")
                    .takes_value(true)
                    .help("Calling plugins to detect vulnerabilities"),
            )
            .arg(
                Arg::with_name("update_plugins")
                    .long("update_plugins")
                    .takes_value(false)
                    .help("Update nuclei plugins"),
            )
            .arg(
                Arg::with_name("update_self")
                    .long("update_self")
                    .takes_value(false)
                    .help("Update self"),
            )
            .arg(
                Arg::with_name("update_fingerprint")
                    .short("u")
                    .long("update_fingerprint")
                    .takes_value(false)
                    .help("Update web fingerprint"),
            );
        let args = app.get_matches();
        let mut stdin: bool = false;
        let mut update_self: bool = false;
        let mut verify_path: String = String::new();
        let mut ips: String = String::new();
        let mut update_fingerprint: bool = false;
        let mut update_plugins: bool = false;
        let mut plugins: String = String::new();
        let mut agent_uuid: String = String::new();
        let mut req_timeout: u64 = 10;
        let mut req_thread: u32 = 100;
        let mut target_url: String = String::new();
        let mut file_path: String = String::new();
        let mut csv_file_path: String = String::new();
        let mut json_file_path: String = String::new();
        let mut proxy_uri: String = String::new();
        if args.is_present("stdin") {
            stdin = true;
        }
        if args.is_present("update_plugins") {
            update_plugins = true;
        }
        if args.is_present("update_self") {
            update_self = true;
        }
        if args.is_present("update_fingerprint") {
            update_fingerprint = true;
        }
        if let Some(nuclei) = args.value_of("plugins") {
            if !has_nuclei_app() {
                println!("Please install nuclei to the environment variable!");
                process::exit(0);
            }
            plugins = nuclei.to_string();
            if !Path::new(&plugins).exists() {
                println!("The plugins directory does not exist!");
                process::exit(0);
            }
        }
        if let Some(target) = args.value_of("target") {
            target_url = target.to_string();
        };
        if let Some(uuid) = args.value_of("uuid") {
            agent_uuid = uuid.to_string();
        };
        if let Some(ip) = args.value_of("ip") {
            ips = ip.to_string();
        };
        if let Some(file) = args.value_of("file") {
            file_path = file.to_string();
        };
        if let Some(verify) = args.value_of("verify") {
            verify_path = verify.to_string();
        };
        if let Some(file) = args.value_of("csv") {
            csv_file_path = file.to_string();
        };
        if let Some(file) = args.value_of("json") {
            json_file_path = file.to_string();
        };
        if let Some(proxy) = args.value_of("proxy") {
            proxy_uri = proxy.to_string();
        };
        if let Some(timeout) = args.value_of("timeout") {
            req_timeout = timeout.parse().unwrap_or(10);
        };
        if let Some(thread) = args.value_of("thread") {
            req_thread = thread.parse().unwrap_or(100);
        };
        WardArgs {
            target: target_url,
            stdin,
            file: file_path,
            update_plugins,
            update_fingerprint,
            verify: verify_path,
            csv: csv_file_path,
            json: json_file_path,
            proxy: proxy_uri,
            timeout: req_timeout,
            plugins,
            update_self,
            thread: req_thread,
            ip: ips,
            uuid: agent_uuid,
        }
    }
}

// https://github.com/0x727/FingerprintHub/releases/download/default/plugins.zip
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Template {
    pub template_id: String,
}

pub fn has_nuclei_app() -> bool {
    return if cfg!(target_os = "windows") {
        Command::new("nuclei.exe")
            .args(["-version"])
            .stdin(Stdio::null())
            .output()
            .is_ok()
    } else {
        Command::new("nuclei")
            .args(["-version"])
            .stdin(Stdio::null())
            .output()
            .is_ok()
    };
}