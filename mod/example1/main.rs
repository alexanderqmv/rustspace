mod other {
    pub mod some;
}

use other::some::do_something;

fn main() {
    do_something();
}
