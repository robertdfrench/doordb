use doors::Client;
use serde_json;
use clap::{Arg, Command};


fn main() {
    // Build the command-line interface using clap
    let matches = Command::new("door_client")
        .version("1.0")
        .author("Your Name")
        .about("Client for interacting with the door server")
        .arg(
            Arg::new("key")
                .short('k')
                .long("key")
                .value_name("KEY")
                .help("Specifies the key")
                .required(true),
        )
        .arg(
            Arg::new("method")
                .short('m')
                .long("method")
                .value_name("METHOD")
                .help("Specifies the method: get, create, delete, increment")
                .required(true),
        )
        .get_matches();

    // Retrieve the key and method from the command-line arguments
    let key = matches.get_one::<String>("key").unwrap().clone();
    let method_str = matches.get_one::<String>("method").unwrap();

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

    // Construct the Query object
    let query = doordb::Query { key, method };

    // Serialize the Query object to JSON
    let query_bytes = serde_json::to_vec(&query).expect("Failed to serialize query");

    // Open the door at /var/run/doordb.door
    let door = Client::open(doordb::PATH).expect("Failed to open door");

    // Send the serialized Query to the server and receive the response
    let response = door.call_with_data(&query_bytes).expect("Door call failed");

    // Deserialize the response into a Result<u8, String>
    let response: Result<u8, String> =
        serde_json::from_slice(response.data()).expect("Failed to deserialize response");

    // Handle the response
    match response {
        Ok(value) => {
            println!("Success: The current value is {}", value);
        }
        Err(err_msg) => {
            eprintln!("Error: {}", err_msg);
            std::process::exit(1);
        }
    }
}
