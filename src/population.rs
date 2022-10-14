use macroquad::{prelude::*, rand::gen_range};

use crate::{nn::NN, world::World};

#[derive(Default)]
pub struct Population {
    size: usize,
    gen: i32,
    pub worlds: Vec<World>,
}

impl Population {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            worlds: (0..size)
                .map(|_| World::simulate(NN::new(vec![89, 16, 4])))
                .collect(),
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        let mut alive = false;
        for world in &mut self.worlds {
            if !world.over {
                alive = true;
                world.update();
            }
        }
        if !alive {
            self.gen += 1;
            println!("{}", self.gen);
            self.next_gen();
        }
    }

    pub fn draw(&self) {
        for world in &self.worlds {
            if !world.over {
                world.draw();
                draw_text(
                    &format!("Gen: {}", self.gen),
                    -100. + screen_width() * 0.5,
                    30. - screen_height() * 0.5,
                    32.,
                    WHITE,
                );
            }
        }
    }

    pub fn next_gen(&mut self) {
        let total = self.worlds.iter().fold(0., |acc, x| acc + x.fitness());
        self.worlds
            .sort_by(|a, b| b.fitness().partial_cmp(&a.fitness()).unwrap());
        let mut new_worlds = (0..self.size / 10)
            .map(|i| World::simulate(self.worlds[i].see_brain().to_owned()))
            .collect::<Vec<_>>();
        // println!(
        //     "Total fitness: {} {} {}",
        //     total,
        //     self.worlds[0].fitness(),
        //     self.worlds[1].fitness()
        // );

        while new_worlds.len() < self.size {
            let rands = (gen_range(0., total), gen_range(0., total));
            // println!("rands: {} {}", rands.0, rands.1);
            let mut sum = 0.;
            let (mut a, mut b) = (None, None);
            for world in &self.worlds {
                sum += world.fitness();
                if a.is_none() && sum >= rands.0 {
                    a = Some(world.see_brain());
                }
                if b.is_none() && sum >= rands.1 {
                    b = Some(world.see_brain());
                }
            }
            // println!("{}", &a.unwrap().weights[0]);
            // println!("{}", &b.unwrap().weights[0]);
            let mut new_brain = NN::crossover(a.unwrap(), b.unwrap());
            // println!("{}", &a.unwrap().weights[0]);
            // println!("{}", &b.unwrap().weights[0]);
            // println!("{}", &new_brain.weights[0]);
            new_brain.mutate();
            new_worlds.push(World::simulate(new_brain));
        }
        self.worlds = new_worlds;
    }
}
