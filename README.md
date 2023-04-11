# Vial
Vial is a chat application written in Rust to be used throught the command line interface.

# Structure

## High-Level 
Vial is structure in the following tree:
- client
- db 
- server
- tests 

## Infrastrure
The plan is to build this project using a queue to receive message like `Apache Kafka` and to  leverage `Redis` to maintain the messages from a specific period of time. Maybe using `Kubernets` for resource management on the server machine using `Kafka` and `Redis` in `docker` containers.

# Development
To run the application you should run `cargo run`

# How to use
The plan is to use commands to send messages through the command line interface. Kinda like the `vim` text editor. Maybe I will write a UI on the some frontend framework like `React.js` or something else.