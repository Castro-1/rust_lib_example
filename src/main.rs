use external_dependencies::{Credentials};

fn main() {
    let creds = Credentials {
        username: String::from("Juanes"),
        password: String::from("Castro"),
    };

    external_dependencies::authenticate(creds);
}