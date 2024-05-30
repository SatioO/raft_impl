use std::{env, process};

use crate::raft::ClusterMember;

#[derive(Debug, Default)]
pub struct Config {
    pub index: usize,
    pub id: String,
    pub address: String,
    pub http: String,
    pub cluster: Vec<ClusterMember>,
}

impl Config {
    pub fn new() -> Self {
        let mut config = Config::default();

        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--node" => {
                    if let Some(node) = args.next() {
                        match node.parse::<usize>() {
                            Ok(index) => config.index = index,
                            Err(_) => {
                                println!("Expected $value to be a valid integer in `--node $value`, got: {}", node);
                                process::exit(1);
                            }
                        }
                    } else {
                        println!("Missing value for parameter: --node");
                        process::exit(1);
                    }
                }
                "--http" => {
                    if let Some(http) = args.next() {
                        config.http = http;
                    } else {
                        println!("Missing value for parameter: --http");
                        process::exit(1)
                    }
                }
                "--cluster" => {
                    if let Some(cluster) = args.next() {
                        for part in cluster.split(';') {
                            let id_address: Vec<&str> = part.split(',').collect();
                            if id_address.len() != 2 {
                                println!(
                                    "Expected $id,$ip format in `--cluster $id,$ip`, got: {}",
                                    part
                                );
                                process::exit(1);
                            }
                            match id_address[0].parse::<u64>() {
                                Ok(id) => {
                                    config.cluster.push(ClusterMember {
                                        id,
                                        address: id_address[1].to_string(),
                                        ..Default::default()
                                    });
                                }
                                Err(_) => {
                                    println!("Expected $id to be a valid integer in `--cluster $id,$ip`, got: {}", id_address[0]);
                                    process::exit(1);
                                }
                            }
                        }
                    } else {
                        println!("Missing value for parameter: --cluster");
                        process::exit(1);
                    }
                }
                _ => {
                    println!("Unknown parameter: {}", arg);
                    process::exit(1);
                }
            }
        }

        if config.index == 0 {
            eprintln!("Missing required parameter: --node $index");
            process::exit(1);
        }

        if config.http.is_empty() {
            eprintln!("Missing required parameter: --http $address");
            process::exit(1);
        }

        if config.cluster.is_empty() {
            eprintln!("Missing required parameter: --cluster $node1Id,$node1Address;...;$nodeNId,$nodeNAddress");
            process::exit(1);
        }

        config
    }
}
