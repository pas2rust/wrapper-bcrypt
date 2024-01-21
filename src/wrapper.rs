use bcrypt::BcryptError;
use darth_rust::DarthRust;
type BcryptHashResult = Result<String, BcryptError>;
type BcryptVerifyResult = Result<bool, BcryptError>;

#[derive(Debug, DarthRust)]
pub struct WrapperBcrypt {
    pub password: String,
    pub cost: u32,
    pub hash: Option<String>,
}

pub trait BcryptTrait {
    fn encode(&self) -> BcryptHashResult;
    fn verify(&self) -> BcryptVerifyResult;
}

impl BcryptTrait for WrapperBcrypt {
    fn encode(&self) -> BcryptHashResult {
        let password = &self.password;
        let cost = self.cost;
        bcrypt::hash(password, cost)
    }
    fn verify(&self) -> BcryptVerifyResult {
        let password = &self.password;
        let hash = self.hash.as_ref().expect("hash must be provided");
        bcrypt::verify(password, hash)
    }
}
