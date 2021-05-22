mod error;
mod mc;
use bytelines::*;
use error::PodmanError;
use mc::Container;
use regex::Regex;
use std::fmt;
use std::io::{Error, ErrorKind};
use std::process::{Command, Output, Stdio};
use std::str;

#[derive(Debug)]
pub enum Runtime {
    Podman,
    Docker,
}

impl fmt::Display for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Instance {
    runtime: String,
    prefix: String,
    image: String,
}

impl Instance {
    ///////////////////////////////////////////////
    //          New Container Instance           //
    ///////////////////////////////////////////////
    pub fn new(runtime: Runtime) -> Self {
        Instance {
            runtime: runtime.to_string().to_lowercase(),
            prefix: String::from("mcli-"),
            image: String::from("itzg/minecraft-server:latest"),
        }
    }

    ///////////////////////////////////////////////
    //       Create new Minecraft Container      //
    ///////////////////////////////////////////////
    pub fn create(
        &self,
        name: &str,
        mode: &str,
        level_type: &str,
        heap: &str,
    ) -> Result<String, PodmanError> {
        let command = Command::new(&self.runtime)
            .args(&["run", "-d"])
            .args(&["-p", "25565"])
            .args(&["--restart", "unless-stopped"])
            .args(&["-e", &format!("MODE={}", mode)])
            .args(&["-e", &format!("LEVEL_TYPE={}", level_type)])
            .args(&["-e", &format!("MEMORY={}", heap)])
            .args(&["-l", &format!("game_mode={}", mode)])
            .args(&["-l", &format!("level_type={}", level_type)])
            .args(&["-e", "EULA=TRUE"])
            .args(&["--name", &format!("{}{}", self.prefix, name)])
            .args(&["-v", &format!("{}{}:/data", self.prefix, name)])
            .arg(&self.image)
            .stdout(Stdio::null())
            .output()?;

        match command.status.success() {
            true => Ok(name.to_string().clone()),
            _ => Err(PodmanError::Command(Error::new(
                ErrorKind::Other,
                String::from_utf8(command.stderr).unwrap(),
            ))),
        }
    }

    ///////////////////////////////////////////////
    //         Remove Minecraft Container        //
    ///////////////////////////////////////////////
    pub fn remove(&self, name: &str) -> Result<String, PodmanError> {
        let stop_container = Command::new(&self.runtime)
            .args(&["stop", &format!("{}{}", self.prefix, name)])
            .stdout(Stdio::null())
            .output()?;

        let remove_container = Command::new(&self.runtime)
            .args(&["rm", &format!("{}{}", self.prefix, name)])
            .stdout(Stdio::null())
            .output()?;

        let remove_volume = Command::new(&self.runtime)
            .args(&["volume", "remove", &format!("{}{}", self.prefix, name)])
            .stdout(Stdio::null())
            .output()?;

        let output: [Output; 3] = [stop_container, remove_container, remove_volume];

        for command in output.iter() {
            if !command.status.success() {
                return Err(PodmanError::Command(Error::new(
                    ErrorKind::Other,
                    String::from_utf8(command.stderr.clone()).unwrap(),
                )));
            }
        }

        Ok(name.to_string().clone())
    }

    ///////////////////////////////////////////////
    //         Start Minecraft Container         //
    ///////////////////////////////////////////////
    pub fn start(&self, name: &str) -> Result<String, PodmanError> {
        let command = Command::new(&self.runtime)
            .args(&["start", &format!("{}{}", self.prefix, name)])
            .stdout(Stdio::null())
            .output()?;

        match command.status.success() {
            true => Ok(name.to_string().clone()),
            _ => Err(PodmanError::Command(Error::new(
                ErrorKind::Other,
                String::from_utf8(command.stderr).unwrap(),
            ))),
        }
    }

    ///////////////////////////////////////////////
    //          Stop Minecraft Container         //
    ///////////////////////////////////////////////
    pub fn stop(&self, name: &str) -> Result<String, PodmanError> {
        let command = Command::new(&self.runtime)
            .args(&["stop", &format!("{}{}", self.prefix, name)])
            .stdout(Stdio::null())
            .output()?;

        match command.status.success() {
            true => Ok(name.to_string().clone()),
            _ => Err(PodmanError::Command(Error::new(
                ErrorKind::Other,
                String::from_utf8(command.stderr).unwrap(),
            ))),
        }
    }

    ///////////////////////////////////////////////
    //         Restart Minecraft Container       //
    ///////////////////////////////////////////////
    pub fn restart(&self, name: &str) -> Result<String, PodmanError> {
        let command = Command::new(&self.runtime)
            .args(&["restart", &format!("{}{}", self.prefix, name)])
            .stdout(Stdio::null())
            .output()?;

        match command.status.success() {
            true => Ok(name.to_string().clone()),
            _ => Err(PodmanError::Command(Error::new(
                ErrorKind::Other,
                String::from_utf8(command.stderr).unwrap(),
            ))),
        }
    }

    ///////////////////////////////////////////////
    //             Start RCON Session            //
    ///////////////////////////////////////////////
    pub fn rcon(&self, name: &str) -> Result<(), PodmanError> {
        Command::new(&self.runtime)
            .args(&[
                "exec",
                "-it",
                &format!("{}{}", self.prefix, name),
                "rcon-cli",
            ])
            .status()?;

        Ok(())
    }

    ///////////////////////////////////////////////
    //               Start Log Tail              //
    ///////////////////////////////////////////////
    pub fn logs(&self, name: &str) -> Result<(), PodmanError> {
        Command::new(&self.runtime)
            .args(&["logs", "-f", &format!("{}{}", self.prefix, name)])
            .status()?;

        Ok(())
    }

    ///////////////////////////////////////////////
    //         List Minecraft Containers         //
    ///////////////////////////////////////////////
    pub fn list(&self) -> Result<Vec<Container>, PodmanError> {
        let command = Command::new(&self.runtime)
            .args(&["ps", "-a", "--format", "table {{.Names}};{{.Ports}};{{.Status}};{{.CreatedAt}};{{.Labels.level_type}};{{.Labels.game_mode}}"])
            .output()?;

        let mut lines = command.stdout.byte_lines();

        let mut containers: Vec<Container> = Vec::new();

        while let Some(line) = lines.next() {
            let new_line = line.unwrap();
            let container_data = str::from_utf8(new_line).unwrap();

            let mut vec: Vec<&str> = container_data.split(";").collect();
            let regex_prefix_str = format!(r"^{}(.*)", self.prefix);
            let re_name = Regex::new(&regex_prefix_str).unwrap();
            let re_port = Regex::new(r"(?:\d+[\.:]){4}(\d+)->25565/tcp").unwrap();

            if re_name.is_match(&vec[0]) {
                vec[0] = re_name.captures(&vec[0]).unwrap().get(1).unwrap().as_str();
                vec[1] = re_port.captures(&vec[1]).unwrap().get(1).unwrap().as_str();
                containers.push(Container::new(vec));
            }
        }

        match command.status.success() {
            true => Ok(containers),
            _ => Err(PodmanError::Command(Error::new(
                ErrorKind::Other,
                String::from_utf8(command.stderr).unwrap(),
            ))),
        }
    }
}
