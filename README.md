# Welcome to Sunberry!</h1>
This is a place, where you can program stuff on a solar-powered Raspberry Pi Zero 2 which will hopefully run 24/7.  

## Prerequisites
```sh
sudo apt install rustc
sudo apt install cargo
```

## Webserver
### Run the webserver
```sh
cd webserver
cargo run
```
### Build the webserver for production
```sh
cargo build --release
```