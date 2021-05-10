use bytelines::*;
use regex::Regex;
use std::process;
use std::process::{Command, Stdio};
use std::str;
use structopt::StructOpt;

#[macro_use(row)]
extern crate tabular;
use tabular::Table;

const DOCKERIMAGE: &str = "itzg/minecraft-server:latest";
const PREFIX: &str = "mcli-";

fn banner() {
    println!("");
    println!("  ███╗   ███╗██╗███╗   ██╗███████╗ ██████╗██████╗  █████╗ ███████╗████████╗");
    println!("  ████╗ ████║██║████╗  ██║██╔════╝██╔════╝██╔══██╗██╔══██╗██╔════╝╚══██╔══╝");
    println!("  ██╔████╔██║██║██╔██╗ ██║█████╗  ██║     ██████╔╝███████║█████╗     ██║   ");
    println!("  ██║╚██╔╝██║██║██║╚██╗██║██╔══╝  ██║     ██╔══██╗██╔══██║██╔══╝     ██║   ");
    println!("  ██║ ╚═╝ ██║██║██║ ╚████║███████╗╚██████╗██║  ██║██║  ██║██║        ██║   ");
    println!("  ╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝        ╚═╝   ");
}

///////////////////////////////////////////////
//                 StructOpt                 //
///////////////////////////////////////////////

#[derive(StructOpt, Debug)]
#[structopt(name = "mcli")]
enum Mcli {
    #[structopt(name = "create", about = "Create a Minecraft server.")]
    Game {
        #[structopt(
            short = "h",
            long = "heap",
            default_value = "1G",
            help = "Sets the maximum Java memory-heap limit. Support format/units as <size>[g|G|m|M|k|K] "
        )]
        heap: String,

        #[structopt(
            short = "m",
            long = "mode",
            default_value = "survival",
            help = "Minecraft game mode. Possible values: creative, survival, adventure"
        )]
        mode: String,

        #[structopt(
            short = "t",
            long = "type",
            default_value = "default",
            help = "Minecraft map type. Possible values: default, flat, largebiomes, amplified, buffet"
        )]
        level_type: String,

        #[structopt(help = "The name of the server")]
        name: String,
    },

    #[structopt(name = "remove", about = "Remove a Minecraft server.")]
    Remove {
        #[structopt(help = "The name of the server")]
        name: String,
    },

    #[structopt(name = "list", about = "List all Minecraft servers.")]
    List {},

    #[structopt(name = "start", about = "Start a Minecraft servers.")]
    Start { name: String },

    #[structopt(name = "stop", about = "Stop a Minecraft servers.")]
    Stop { name: String },

    #[structopt(name = "restart", about = "Restart a Minecraft servers.")]
    Restart { name: String },

    #[structopt(name = "rcon", about = "Start a rcon session to a Minecraft servers.")]
    Rcon { name: String },

    #[structopt(name = "logs", about = "Trail logs from a Minecraft servers.")]
    Logs { name: String },
}

///////////////////////////////////////////////
//               Main Function               //
///////////////////////////////////////////////

fn main() {
    let crun: &str;
    let podman = Command::new("podman")
        .arg("--version")
        .stdout(Stdio::null())
        .status();
    let docker = Command::new("docker")
        .arg("--version")
        .stdout(Stdio::null())
        .status();

    if let Ok(_x) = podman {
        crun = "podman";
    } else if let Ok(_x) = docker {
        crun = "docker";
    } else {
        println!("No container runtime found! Aborting!");
        process::exit(1);
    }

    match Mcli::from_args() {
        Mcli::Game {
            name,
            mode,
            level_type,
            heap,
        } => {
            Command::new(&crun)
                .args(&["run", "-d"])
                .args(&["-p", "25565"])
                .args(&["--restart", "unless-stopped"])
                .args(&["-e", &format!("MODE={}", mode)])
                .args(&["-e", &format!("LEVEL_TYPE={}", level_type)])
                .args(&["-e", &format!("MEMORY={}", heap)])
                .args(&["-l", &format!("game_mode={}", mode)])
                .args(&["-l", &format!("level_type={}", level_type)])
                .args(&["-e", "EULA=TRUE"])
                .args(&["--name", &format!("{}{}", &PREFIX, name)])
                .args(&["-v", &format!("{}{}:/data", &PREFIX, name)])
                .arg(DOCKERIMAGE)
                .stdout(Stdio::null())
                .status()
                .expect("process failed to execute");
        }

        Mcli::Remove { name } => {
            Command::new(&crun)
                .args(&["stop", &format!("{}{}", &PREFIX, name)])
                .stdout(Stdio::null())
                .status()
                .expect("Could not stop docker container");

            Command::new(&crun)
                .args(&["rm", &format!("{}{}", &PREFIX, name)])
                .stdout(Stdio::null())
                .status()
                .expect("Could not remove docker container");

            Command::new(&crun)
                .args(&["volume", "remove", &format!("{}{}", &PREFIX, name)])
                .stdout(Stdio::null())
                .status()
                .expect("Could not remove docker volume");
        }

        Mcli::Start { name } => {
            Command::new(&crun)
                .args(&["start", &format!("{}{}", &PREFIX, name)])
                .stdout(Stdio::null())
                .status()
                .expect("Could not start container");
        }

        Mcli::Stop { name } => {
            Command::new(&crun)
                .args(&["stop", &format!("{}{}", &PREFIX, name)])
                .stdout(Stdio::null())
                .status()
                .expect("Could not stop container");
        }

        Mcli::Restart { name } => {
            Command::new(&crun)
                .args(&["restart", &format!("{}{}", &PREFIX, name)])
                .stdout(Stdio::null())
                .status()
                .expect("Could not restart container");
        }

        Mcli::Rcon { name } => {
            Command::new(&crun)
                .args(&["exec", "-it", &format!("{}{}", &PREFIX, name), "rcon-cli"])
                .status()
                .expect("Could not start rcon session");
        }

        Mcli::Logs { name } => {
            Command::new(&crun)
                .args(&["logs", "-f", &format!("{}{}", &PREFIX, name)])
                .status()
                .expect("Could not trail logs.");
        }

        Mcli::List {} => {
            banner();
            let containers = Command::new(&crun)
        .args(&["ps", "-a", "--format", "table {{.Status}};{{.Ports}};{{.Names}};{{.CreatedAt}};{{.Labels.level_type}};{{.Labels.game_mode}}"])
        .output()
        .expect("docker ps failed to execute");

            let mut table = Table::new(" {:<}   {:<}   {:<}   {:<}   {:<}   {:<}");
            table.add_row(row!("Name", "Mode", "Type", "Port", "Status", "Created"));

            let mut lines = containers.stdout.byte_lines();

            while let Some(line) = lines.next() {
                let char_string = str::from_utf8(line.unwrap()).unwrap();
                let vec: Vec<&str> = char_string.split(";").collect();

                if check_if_mcli(vec[2]) {
                    let mc_container = McContainer::new(vec);
                    table.add_row(row!(
                        mc_container.name,
                        capitalize_first(&mc_container.game_mode),
                        capitalize_first(&mc_container.level_type),
                        mc_container.port,
                        mc_container.status,
                        mc_container.created
                    ));
                }
            }
            print!("\n{}\n", table);
        }
    }
}

fn check_if_mcli(name: &str) -> bool {
    let re = Regex::new(r"^mcli-.*").unwrap();
    re.is_match(name)
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    chars
        .next()
        .map(|first_letter| first_letter.to_uppercase())
        .into_iter()
        .flatten()
        .chain(chars)
        .collect()
}

///////////////////////////////////////////////
//            McContainer Struct             //
///////////////////////////////////////////////

#[derive(Debug)]
struct McContainer {
    status: String,
    port: String,
    name: String,
    created: String,
    level_type: String,
    game_mode: String,
}

impl McContainer {
    fn new(container: Vec<&str>) -> Self {
        McContainer {
            status: String::from(container[0]),
            port: McContainer::extract_port(container[1]),
            name: McContainer::remove_prefix(container[2]),
            created: String::from(container[3]),
            level_type: String::from(container[4]),
            game_mode: String::from(container[5]),
        }
    }

    fn remove_prefix(name: &str) -> String {
        let regex_string: String = format!("^{}([a-zA-Z0-9-_]+)", &PREFIX);
        let re = Regex::new(&regex_string).unwrap();
        let cap = re.captures(name).unwrap();
        cap.get(1).map_or("", |m| m.as_str()).to_string()
    }

    fn extract_port(port: &str) -> String {
        let re = Regex::new(r"(?:\d+[\.:]){4}(\d+)->25565/tcp").unwrap();
        match re.captures(port) {
            Some(x) => x.get(1).map_or("", |m| m.as_str()).to_string(),
            _ => "".to_string(),
        }
    }
}
