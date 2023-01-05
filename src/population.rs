use macroquad::{prelude::*, rand::gen_range};

use crate::{nn::NN, world::World};

#[derive(Default)]
pub struct Population {
    size: usize,
    gen: i32,
    best: bool,
    pub worlds: Vec<World>,
}

impl Population {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            worlds: (0..size).map(|_| World::simulate(None)).collect(),
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
            self.next_gen();
        }
        if is_key_pressed(KeyCode::Z) {
            self.best = !self.best;
        }
    }

    pub fn draw(&self) {
        for world in self.worlds.iter().rev() {
            if self.best {
                if world.player.color.is_some() {
                    world.draw();
                }
            } else if !world.over {
                world.draw();
            }
        }
        draw_text(
            &format!("Gen: {}", self.gen),
            -150. + screen_width() * 0.5,
            30. - screen_height() * 0.5,
            32.,
            WHITE,
        );
    }

    pub fn next_gen(&mut self) {
        let total = self.worlds.iter().fold(0., |acc, x| acc + x.fitness);
        self.worlds
            .sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        for i in &self.worlds {
            println!("Fitness: {}", i.fitness);
        }
        println!("Gen: {}, Fitness: {}", self.gen, self.worlds[0].fitness);
        let mut new_worlds = (0..self.size / 20)
            .map(|i| World::simulate(Some(self.worlds[i].see_brain().to_owned())))
            .collect::<Vec<_>>();
        new_worlds[0].set_best();
        while new_worlds.len() < self.size {
            let rands = (gen_range(0., total), gen_range(0., total));
            let mut sum = 0.;
            let (mut a, mut b) = (None, None);
            for world in &self.worlds {
                sum += world.fitness;
                if a.is_none() && sum >= rands.0 {
                    a = Some(world.see_brain());
                }
                if b.is_none() && sum >= rands.1 {
                    b = Some(world.see_brain());
                }
            }
            if a.is_none() {
                a = Some(self.worlds.last().unwrap().see_brain());
            }
            if b.is_none() {
                b = Some(self.worlds.last().unwrap().see_brain());
            }
            let mut new_brain = NN::crossover(a.unwrap(), b.unwrap());
            new_brain.mutate();
            new_worlds.push(World::simulate(Some(new_brain)));
        }
        self.worlds = new_worlds;
    }
}
