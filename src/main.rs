mod common_concepts;
mod ownership;
mod structs;
mod enums;

use common_concepts::learn_common_concepts;
use ownership::learn_ownership;
use structs::learn_structs;
use enums::learn_enums_match;

fn main() {
    learn_common_concepts();
    learn_ownership();
    learn_structs();
    learn_enums_match();
}

