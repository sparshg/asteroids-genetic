use crate::world::World;

#[derive(Default)]
struct Population {
    size: i32,
    gen: i32,
    worlds: Vec<World>,
}

impl Population {
    // pub fn new(size: i32) -> Self {
    //     Self {
    //         size: size,
    //         worlds: vec![World::new(); size],
    //         ..Default::default(),
    //     }
    // }
}
