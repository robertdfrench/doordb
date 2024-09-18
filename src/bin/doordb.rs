use clap::{Arg, Command};

fn main() {
    // Build the command-line interface using clap
    let matches = Command::new("doordb")
        .version("1.0")
        .about("Manage shared counters")
        .arg(
            Arg::new("method")
                .value_name("METHOD")
                .help("Specifies the method: get, create, delete, increment")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("key")
                .value_name("KEY")
                .help("Specifies the key")
                .required(true)
                .index(2),
        )
        .get_matches();

    // Retrieve the key and method from the command-line arguments
    let method_str = matches.get_one::<String>("method").unwrap();
    let key = matches.get_one::<String>("key").unwrap().clone();

    // Parse the method string into the doordb::Method enum
    let method = match method_str.to_lowercase().as_str() {
        "get" => doordb::Method::Get,
        "create" => doordb::Method::Create,
        "delete" => doordb::Method::Delete,
        "increment" => doordb::Method::Increment,
        _ => {
            eprintln!("Invalid method: {}", method_str);
            std::process::exit(1);
        }
    };

    let client = doordb::Client::new();
    let response = client.submit_query(method, key);

    // Handle the response
    match response {
        Ok(value) => {
            println!("{}", value);
        }
        Err(err_msg) => {
            eprintln!("Error: {}", err_msg);
            std::process::exit(1);
        }
    }
}
