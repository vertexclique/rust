use sys::size_of;
extern mod std;

struct Cat {
    x: int
}

struct Kitty {
    x: int,
    drop {}
}

fn main() {
    assert (size_of::<Cat>() == size_of::<int>());
    assert (size_of::<Kitty>() == size_of::<(int, i8)>());
}
