
mod tst_structs;
mod utils;
mod tst_vector;

use tst_vector as V;


fn main() {
    tst_structs::run_tst_structs();
    utils::separate();
    V::test1();
    
}
