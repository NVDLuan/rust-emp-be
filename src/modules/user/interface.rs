pub trait UserRepository {
    fn create_user(&self, username: &str, password: &str) -> Result<(), String>;
    fn get_user(&self, username: &str) -> Result<Option<User>, String>;
}

pub trait UserService {
    fn register_user(&self, username: &str, password: &str) -> Result<(), String>;
    fn authenticate_user(&self, username: &str, password: &str) -> Result<bool, String>;
}