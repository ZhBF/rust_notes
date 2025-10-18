mod common_concepts;
mod ownership;
mod structs;

use common_concepts::learn_common_concepts;
use ownership::learn_ownership;
use structs::learn_structs;

fn main() {
    learn_common_concepts();
    learn_ownership();
    learn_structs();
}

