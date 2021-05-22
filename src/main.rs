use colored::*;
use std::process;
use std::process::{Command, Stdio};
use std::str;
use structopt::StructOpt;

// mod mc_container;
mod podman;
use podman::{Instance, Runtime};

#[macro_use(row)]
extern crate tabular;
use tabular::Table;

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
    let container: Instance;

    let podman = Command::new("podman")
        .arg("--version")
        .stdout(Stdio::null())
        .status();
    let docker = Command::new("docker")
        .arg("--version")
        .stdout(Stdio::null())
        .status();

    if let Ok(_x) = podman {
        container = Instance::new(Runtime::Podman);
    } else if let Ok(_x) = docker {
        container = Instance::new(Runtime::Docker);
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
        } => match container.create(&name, &mode, &level_type, &heap) {
            Ok(server_name) => println!("{} Created new server :: {}", "[+]".green(), server_name),
            Err(err) => println!("{} {}", "[-]".red(), err),
        },

        Mcli::Remove { name } => match container.remove(&name) {
            Ok(server_name) => println!("{} Removed server :: {}", "[+]".green(), server_name),
            Err(err) => println!("{} {}", "[-]".red(), err),
        },

        Mcli::Start { name } => match container.start(&name) {
            Ok(server_name) => println!("{} Started server :: {}", "[+]".green(), server_name),
            Err(err) => println!("{} {}", "[-]".red(), err),
        },

        Mcli::Stop { name } => match container.stop(&name) {
            Ok(server_name) => println!("{} Stopped server :: {}", "[+]".green(), server_name),
            Err(err) => println!("{} {}", "[-]".red(), err),
        },

        Mcli::Restart { name } => match container.restart(&name) {
            Ok(server_name) => println!("{} Restarted server :: {}", "[+]".green(), server_name),
            Err(err) => println!("{} {}", "[-]".red(), err),
        },

        Mcli::Rcon { name } => {
            container.rcon(&name).unwrap();
        }

        Mcli::Logs { name } => {
            container.logs(&name).unwrap();
        }

        Mcli::List {} => match container.list() {
            Ok(containers) => {
                banner();
                let mut table = Table::new(" {:<}   {:<}   {:<}   {:<}   {:<}   {:<}");
                table.add_row(row!("Name", "Mode", "Type", "Port", "Status", "Created"));

                containers.iter().for_each(|x| {
                    table.add_row(row!(
                        x.get("name"),
                        x.get("game_mode"),
                        x.get("level_type"),
                        x.get("port"),
                        x.get("status"),
                        x.get("created"),
                    ));
                });
                print!("\n{}\n", table);
            }
            Err(err) => println!("{} {}", "[-]".red(), err),
        },
    }
}
