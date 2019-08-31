pub trait AccountService: Clone {
    fn find_user(&self, username: &str, password: &str) -> &str;
}

pub trait UserAccount {
    fn user_id(&self) -> &str;
}
