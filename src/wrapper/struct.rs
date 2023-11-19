use std::collections::HashMap;

struct Application {
    name: String,
    language: String,
    version: String,
}

struct User {
    id: u16,
    name: String
}

fn create_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("name".to_string(), "Jarvis".to_string());
    map.insert("language".to_string(), "python".to_string());
    map.insert("version".to_string(), "4.0".to_string());
    map
}

fn main() {
    let map = create_map();
    let name = map.get("name").unwrap().to_string();
    let language = map.get("language").unwrap().to_string();
    let version = map.get("version").unwrap().to_string();
    let app_info = Application { name, language, version };
    println!("App name: {}", app_info.name);
    println!("App language: {}", app_info.language);
    println!("App version: {}", app_info.version);
    let id: u16 = 12345;
    let name = String::from("Vignesh");
    let user_info = User { id, name };
    println!("User ID: {}", user_info.id);
    println!("User Name: {}", user_info.name)
}
