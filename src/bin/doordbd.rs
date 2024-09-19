use doors::server::Door;
use doors::UCred;
use doors::illumos::door_h;
use serde_json;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use libc;


fn main() {
    // Create a shared BTreeMap protected by a Mutex
    let map = Arc::new(Mutex::new(BTreeMap::<String, Node>::new()));

    // Convert the Arc to a raw pointer and pass it as the cookie
    let map_ptr = Arc::into_raw(map);

    // Open the door at /var/run/doordb.door
    let door = Door::create_with_cookie(server_proc, map_ptr as u64)
        .expect("Failed to create door");

    door.force_install(doordb::PATH)
        .expect("Faily to install door");

    // Keep the main thread alive
    println!("DoorDB server is running...");
    loop {
        std::thread::park();
    }
}

struct Node {
    owner: libc::uid_t,
    count: u64
}

extern "C" fn server_proc(
    cookie: *const libc::c_void,
    argp: *const libc::c_char,
    arg_size: libc::size_t,
    _dp: *const door_h::door_desc_t,
    _n_desc: libc::c_uint
) {
    // Reconstruct the Arc from the raw pointer
    let map_arc = unsafe { Arc::from_raw(cookie as *const Mutex<BTreeMap<String, Node>>) };

    // Clone the Arc to increase the reference count
    let map_arc_clone = Arc::clone(&map_arc);

    // Prevent the original Arc from being dropped by converting it back into a raw pointer
    let _ = Arc::into_raw(map_arc);

    // Deserialize the Query object from data
    let data = unsafe { std::slice::from_raw_parts(argp as *const u8, arg_size) };
    let query: doordb::Query = serde_json::from_slice(data).map_err(|e| {
        eprintln!("Failed to deserialize query: {}", e);
        e
    }).unwrap();

    let client_credentials = UCred::new().unwrap();
    let client_uid = client_credentials.euid().unwrap();

    // Handle the query and interact with the shared map
    let result = {
        let mut map = map_arc_clone.lock().unwrap();

        match query.method {
            doordb::Method::Get => {
                if let Some(node) = map.get(&query.key) {
                    if node.owner == client_uid {
                        Ok(node.count)
                    } else {
                        Err("EPERM".to_string())
                    }
                } else {
                    Err("Key not found".to_string())
                }
            }
            doordb::Method::Create => {
                if map.contains_key(&query.key) {
                    Err("Key already exists".to_string())
                } else {
                    let node = Node {
                        owner: client_uid,
                        count: 0
                    };
                    map.insert(query.key, node);
                    Ok(0)
                }
            }
            doordb::Method::Delete => {
                if let Some(node) = map.get(&query.key) {
                    if node.owner == client_uid {
                        if let Some(node) = map.remove(&query.key) {
                            Ok(node.count)
                        } else {
                            // We have an exclusive lock and just confirmed the existence of this
                            // key, so if we can't delete it now, something is very bad.
                            unreachable!();
                        }
                    } else {
                        Err("EPERM".to_string())
                    }
                } else {
                    Err("Key not found".to_string())
                }
            }
            doordb::Method::Increment => {
                if let Some(node) = map.get_mut(&query.key) {
                    if node.owner == client_uid {
                        node.count += 1;
                        Ok(node.count)
                    } else {
                        Err("EPERM".to_string())
                    }
                } else {
                    Err("Key not found".to_string())
                }
            }
        }
    };

    // Serialize the result to JSON
    let response = serde_json::to_vec(&result).unwrap();

    // Return the response
    unsafe {
        door_h::door_return(
            response.as_ptr() as *const libc::c_char,
            response.len(),
            std::ptr::null(),
            0
        );
    }
}
