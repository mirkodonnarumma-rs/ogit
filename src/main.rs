use crate::ogit::initialize_repository::init_repo;

pub mod ogit;

fn main() -> Result<(), std::io::Error>{
    println!("ogit - minimal object store");
    init_repo()
}