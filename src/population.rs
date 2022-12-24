use macroquad::{prelude::*, rand::gen_range};

use crate::nn::NN;
use crate::player::Player;
use crate::world::World;

#[derive(Default)]
pub struct Population {
    size: usize,
    gen: i32,
    // pub worlds: Vec<World>,
    players: Vec<Player>,
    world: World,
}

impl Population {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            players: (0..size).map(|_| Player::new()).collect(),
            world: World::new(200.),
            // worlds: (0..size).map(|_| World::simulate(None)).collect(),
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        self.world.update();
        let mut alive = false;
        for player in self.players.iter_mut() {
            if player.alive {
                alive = true;
                player.update(&self.world.pillars[0]);
                self.world.check_collision(player);
            }
        }
        if !alive {
            self.gen += 1;
            self.next_gen();
        }
    }

    pub fn draw(&self) {
        self.world.draw();
        for player in self.players.iter().filter(|x| x.alive) {
            player.draw();
        }
    }

    pub fn next_gen(&mut self) {
        // let total = self.worlds.iter().fold(0., |acc, x| acc + x.fitness);
        let total = self.players.iter().fold(0, |acc, x| acc + x.lifespan);
        // self.worlds
        //     .sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        self.players.sort_by(|a, b| b.lifespan.cmp(&a.lifespan));
        // for i in &self.worlds {
        //     println!("Fitness: {}", i.fitness);
        // }
        // println!("Gen: {}, Fitness: {}", self.gen, self.worlds[0].fitness);
        println!("Gen: {}, Fitness: {}", self.gen, self.players[0].lifespan);
        // // let mut new_worlds = vec![World::simulate(Some(self.worlds[0].see_brain().to_owned()))];
        let h = self.world.h + 10.;
        self.world = World::new(h.min(200.));
        // let mut new_worlds = (0..self.size / 20)
        //     .map(|i| World::simulate(Some(self.worlds[i].see_brain().to_owned())))
        //     .collect::<Vec<_>>();
        let mut new_players = (0..self.size / 20)
            .map(|i| Player::simulate(self.players[i].brain.clone()))
            .collect::<Vec<_>>();
        new_players[0].color = Some(RED);
        // for _ in 0..self.size / 20 {
        //     new_worlds.push(World::simulate(None));
        // }
        // if is_key_down(KeyCode::K) {
        // new_worlds[0].set_best();
        // }
        // println!(
        //     "Total fitness: {} {} {}",
        //     total,
        //     self.worlds[0].fitness(),
        //     self.worlds[1].fitness()
        // );
        while new_players.len() < self.size {
            let rands = (gen_range(0, total), gen_range(0, total));
            // println!("rands: {} {} {}", rands.0, rands.1, total);
            let mut sum = 0;
            let (mut a, mut b) = (None, None);
            for player in &self.players {
                sum += player.lifespan;
                if a.is_none() && sum >= rands.0 {
                    a = player.brain.as_ref();
                }
                if b.is_none() && sum >= rands.1 {
                    b = player.brain.as_ref();
                }
            }
            // println!("{}", &a.unwrap().weights[0]);
            // println!("{}", &b.unwrap().weights[0]);
            // if a.is_none() {
            //     a = Some(self.worlds.last().unwrap().see_brain());
            // }
            // if b.is_none() {
            //     b = Some(self.worlds.last().unwrap().see_brain());
            // }
            let mut new_brain = NN::crossover(&a.unwrap(), &b.unwrap());
            // println!("{}", &a.unwrap().weights[0]);
            // println!("{}", &b.unwrap().weights[0]);
            new_brain.mutate();
            // println!("{}", &new_brain.weights[0]);
            // new_worlds.push(World::simulate(Some(new_brain)));
            new_players.push(Player::simulate(Some(new_brain)));
        }
        self.players = new_players;
        // self.worlds = new_worlds;
    }
}
