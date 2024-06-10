# Welcome to Sunberry!</h1>
Sunberry is the repository for rust programs running on a solar-powered Raspberry Pi Zero 2 which will hopefully run 24/7.

Overview
- [Introduction](#introduction)
- [Projects](#projects)
  - [Webserver](#webserver)
  - [Collector](#collector)
- [Coding](#coding)
    - [Prerequisites](#prerequisites)
    - [Testing](#testing)
    - [Deployment](#deployment)

## Introduction
There are different projects in this repository. All of them are written in rust and have a project Cargo.toml in /project. Each project has it's own Cargo.toml too, where all the dependencies and stuff are declared. The reason for this multi-project structure is that some projects might use stuff from other projects as well, which is possible with a setup like this.

## Projects

### Webserver
Webserver is the first project created for sunberry. It is a webserver(duh) delivering template pages and parses markdown to websites which then get delivered to the user.

### Collector
The collector *collects* data from the INA219 and INA226 modules which are connected to the sunberry via I²C and writes them into an SQLite database.  

## Coding
Let's go!
### Prerequisites
Install rust by using:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Testing
Cargo is the tool used for compiling the code into an executable.
```sh
cd project/<project_name>
cargo run
```
You can install cargo watch to watch changes as you program, which can be very helpful (when working on the webserver for instance).
```sh
cd project/<project_name>
cargo install cargo-watch
cargo watch -x run
```

### Deployment
When building for production you have to cross-compile the rust code for the aarch64 architecture. There is a deploy script to accomplish this. In order for the script to work, you have to add a sunberry ssh config to your ~/.ssh/config:  
```bash
Host sunberry
    HostName IP will follow
    User sunshine
    IdentityFile /your/ssh_key
```

Deploying to the sunberry is rather straight forward:
```bash
cd project/<project_name>
./deploy
```