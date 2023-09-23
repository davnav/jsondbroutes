use serde_json::{json, Map, Result, Value};
use std::collections::HashMap;
use std::error::Error;

/*
if a give a json, the program should return possible routes with key as the first value
eg:

    {
        "posts": [
            { "id": 1, "title": "json-server", "author": "typicode" },
            { "id": 2, "title": "second-server","author":"Naveen" }
        ],
        "comments": [
            { "id": 1, "body": "some comment", "postId": 1 }
        ],
        "profile": { "name": "typicode" }
    }

-- Routes returns for the above examples are

    /posts/<id>     => /posts/1 should list return the first row
                    => /posts/2 should return the second record from posts
    /comments/<id>
    /profile/


*/

fn json_routes(s: &str, path: &str) -> Value {
    let res: Value = serde_json::from_str(s).unwrap();

    /// extract path to key value
    let path_keymap = path_map(path);
    eprintln!("{:?}", path_keymap);

    /// json match with path map
    /// in order to route , json keys should be in path keys  
    json_keymaps(&res, path_keymap);
    res
}
/// Below function extract keys from json
///
fn json_keymaps(json_value: &Value, path_keymap: HashMap<&str, &str>) {
    let obj: Map<String, Value> = json_value.as_object().unwrap().clone();
    for (key, value) in path_keymap {
        if obj.contains_key(key) {
            // eprintln!("{:?}",obj[key]);
            let v = vec![&obj[key]];
            let mut index = 0;
            for arr_obj in v {

                eprintln!("{:?}",arr_obj[index]["id"].as_str());
                if arr_obj[index]["id"].as_str() == Some(value) {
                    eprintln!("{}", arr_obj[index]);
                }

                index += 1;
            }
        } else {
        }
    }
}

/// path mapping to hashmap
fn path_map(path: &str) -> HashMap<&str, &str> {
    let path_vec: Vec<&str> = path.split('/').collect();
    let mut path_map: HashMap<&str, &str> = HashMap::new();

    for key in path_vec.iter().step_by(2) {
        for value in path_vec[1..].iter().step_by(2) {
            path_map.insert(key, value);
        }
    }

    path_map
}

/// Creating routes
/// if "/posts" as an endpoint, list out the records under "posts" key
/// if "/posts/<id>" as an endpoint, list only the record with the matching id

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_path_id() {
        let test_json: &str = r#" { "posts": [{ "id": "1", "title": "json" , "author": "nav" }]} "#;
        let path = "posts/1";
        let res_json = json_routes(&test_json, path);
        // eprintln!("{:#?}",&res_json);
        assert_eq!(
            res_json["posts"][0],
            json!({
            "author":"nav",
            "id": "1",
             "title": "json"
            })
        );
    }

    #[test]
    fn json_path(){
        let test_json: &str = r#" { "posts": [{ "id": "1", "title": "json" , "author": "nav" }]} "#;
        let path = "posts";
        let res_json = json_routes(&test_json, path);
        // eprintln!("{:#?}",&res_json);
        assert_eq!(
            res_json["posts"],
            json!(
            [{"author":"nav",
            "id": "1",
             "title": "json"}])
        );
    }

}
