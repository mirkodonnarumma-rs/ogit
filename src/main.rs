pub mod ogit;
use std::path::Path;

use crate::ogit::init_repo::init_repo;

fn main() {
    let _ = init_repo();
}
