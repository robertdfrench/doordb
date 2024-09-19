use doors::server::Door;
use doors::UCred;
use doors::illumos::door_h;
use serde_json;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use libc;
use doordb::TextMethod;
use doordb::Query;
use doordb::Response;


fn main() {
    // Create a shared BTreeMap protected by a RwLock
    let map = Arc::new(RwLock::new(BTreeMap::<String, Node>::new()));

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
    let map_arc = unsafe { Arc::from_raw(cookie as *const RwLock<BTreeMap<String, Node>>) };

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

    let result = match query {
        Query::Counter{ key, method } => {
            match method {
                doordb::Method::Get => {
                    let map = map_arc_clone.read().unwrap();

                    if let Some(node) = map.get(&key) {
                        if node.owner == client_uid {
                            Ok(Response::Counter(node.count))
                        } else {
                            Err("EPERM".to_string())
                        }
                    } else {
                        Err("Key not found".to_string())
                    }
                }
                doordb::Method::Create => {
                    let mut map = map_arc_clone.write().unwrap();

                    if map.contains_key(&key) {
                        Err("Key already exists".to_string())
                    } else {
                        let node = Node {
                            owner: client_uid,
                            count: 0
                        };
                        map.insert(key, node);
                        Ok(Response::Counter(0))
                    }
                }
                doordb::Method::Delete => {
                    let mut map = map_arc_clone.write().unwrap();

                    if let Some(node) = map.get(&key) {
                        if node.owner == client_uid {
                            if let Some(node) = map.remove(&key) {
                                Ok(Response::Counter(node.count))
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
                    let mut map = map_arc_clone.write().unwrap();

                    if let Some(node) = map.get_mut(&key) {
                        if node.owner == client_uid {
                            node.count += 1;
                            Ok(Response::Counter(node.count))
                        } else {
                            Err("EPERM".to_string())
                        }
                    } else {
                        Err("Key not found".to_string())
                    }
                }
            }
        },
        Query::Text(method) => match method {
            TextMethod::Delete{ key: _ } => Err("Not implemented".to_string()),
            TextMethod::Read{ key: _ } => Err("Not implemented".to_string()),
            TextMethod::Write{ key: _, value: _ } => Err("Not implemented".to_string()),
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
