enum UserRole {
    RO,
    USER,
    ADMIN,
}

impl UserRole {
    fn name(&self) -> &str {
        match *self {
            UserRole::RO => "read-only",
            UserRole::USER => "user",
            UserRole::ADMIN => "administrator",
        }
    }

    fn is_access_allowed(&self, http_method:&str) -> bool{
        match http_method {
            "HEAD" | "GET" =>true,
            "POST" | "PUT" => {
                *self == UserRole::USER
                || *self == UserRole::ADMIN
            }
            "DELETE"=>*self == UserRole::ADMIN, _ =>false,
        }

    }
}

// fn main() {
//     let role = UserRole::RO;
//     if role.is_access_allowed("POST") {
//         println!("OK:{}",role.name());
//     }else{
//         println!("Access denied:{}", role.name());
//     }
// }