// use example::artistic_concepts::kinds::PrimaryColor;
// use example::artistic_concepts::utils::mix;

// use re-export items
use artistic_concepts::mix;
use artistic_concepts::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
