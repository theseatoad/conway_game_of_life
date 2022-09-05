#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Tile {
    pub on: bool,
}

impl Tile {
    pub fn new(on: bool) -> Tile {
        Tile { on }
    }

    /**
     * Rule 1: Any live cell with two or three live neighbours survives.
     * Rule 2: Any dead cell with three live neighbours becomes a live cell.
     * Rule 3: All other live cells die in the next generation. Similarly, all other dead cells stay dead.
     */
    pub fn evolve(
        self,
        left: Option<&Tile>,
        up: Option<&Tile>,
        right: Option<&Tile>,
        down: Option<&Tile>,
        topleft: Option<&Tile>,
        topright: Option<&Tile>,
        bottomleft: Option<&Tile>,
        bottomright: Option<&Tile>,
    ) -> Tile {
        //Count alive neighbors
        let mut alive_neighbours = 0;
        if left == Some(&Tile::new(true)) {
            alive_neighbours += 1;
        }
        if up == Some(&Tile::new(true)) {
            alive_neighbours += 1;
        }
        if right == Some(&Tile::new(true)) {
            alive_neighbours += 1;
        }
        if down == Some(&Tile::new(true)) {
            alive_neighbours += 1;
        }
        if topleft == Some(&Tile::new(true)) {
            alive_neighbours += 1;
        }
        if topright == Some(&Tile::new(true)) {
            alive_neighbours += 1;
        }
        if bottomleft == Some(&Tile::new(true)) {
            alive_neighbours += 1;
        }
        if bottomright == Some(&Tile::new(true)) {
            alive_neighbours += 1;
        }
        if self.on == true {
            if alive_neighbours == 2 || alive_neighbours == 3 {
                Tile::new(true)
            } else {
                Tile::new(false)
            }
        } else {
            if alive_neighbours == 3 {
                Tile::new(true)
            } else {
                Tile::new(false)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct World {
    pub width: i32,
    pub tiles: Vec<Tile>,
}

impl World {
    pub fn from_data(width: i32, tiles: Vec<Tile>) -> World {
        World {
            width: width,
            tiles: tiles,
        }
    }
    fn get(&self, coordinates: (i32, i32)) -> Option<&Tile> {
        if coordinates.0 >= 0
            && coordinates.0 < self.width
            && coordinates.1 >= 0
            && coordinates.1 < self.width
        {
            Some(&self.tiles[(coordinates.0 + coordinates.1 * self.width) as usize])
        } else {
            None
        }
    }

    pub fn tick(&mut self) {
        let mut new_world: Vec<Tile> = Vec::new();
        for y in 0..self.width {
            for x in 0..self.width {
                let new_tile = self.get((x, y)).unwrap().evolve(
                    self.get((x - 1, y)),
                    self.get((x + 1, y)),
                    self.get((x - 1, y-1)),
                    self.get((x + 1, y-1)),
                    self.get((x - 1, y+1)),
                    self.get((x + 1, y+1)),
                    self.get((x, y+1)),
                    self.get((x, y-1)),
                );
                new_world.push(new_tile.clone());
            }
        }
        self.tiles = new_world
    }
}
#[cfg(test)]
mod tests {
    use crate::{Tile, World};

    #[test]
    fn init_world() {
        let world = World::from_data(
            2,
            vec![
                Tile::new(false),
                Tile::new(true),
                Tile::new(false),
                Tile::new(true),
            ],
        );
        assert_eq!(world.get((0, 0)).unwrap().on, Tile::new(false).on);
        assert_eq!(world.get((1, 0)).unwrap().on, Tile::new(true).on);
        assert_eq!(world.get((0, 1)).unwrap().on, Tile::new(false).on);
        assert_eq!(world.get((1, 1)).unwrap().on, Tile::new(true).on);
    }

    #[test]
    fn tile_rule_1() {
        /*
         * 010
         * 011  (1,1) == on --> (1,1) == on
         * 010
         */
        let world = World::from_data(
            3,
            vec![
                Tile::new(false),
                Tile::new(true),
                Tile::new(false),
                Tile::new(false),
                Tile::new(true),
                Tile::new(true),
                Tile::new(false),
                Tile::new(true),
                Tile::new(false),
            ],
        );
        let (x, y) = (1, 1);
        assert_eq!(
            world.get((x, y)).unwrap().evolve(
                world.get((x - 1, y)),
                world.get((x + 1, y)),
                world.get((x, y - 1)),
                world.get((x, y + 1)),
                world.get((x + 1, y + 1)),
                world.get((x - 1, y + 1)),
                world.get((x + 1, y - 1)),
                world.get((x - 1, y - 1)),
            ),
            Tile::new(true)
        )
    }

    #[test]
    fn tile_rule_1_with_3_neighbour() {
        /*
         * 010
         * 011  (1,1) == on --> (1,1) == on
         * 010
         */
        let world = World::from_data(
            3,
            vec![
                Tile::new(false),
                Tile::new(true),
                Tile::new(false),
                Tile::new(false),
                Tile::new(true),
                Tile::new(true),
                Tile::new(false),
                Tile::new(true),
                Tile::new(false),
            ],
        );
        let (x, y) = (1, 1);
        assert_eq!(
            world.get((x, y)).unwrap().evolve(
                world.get((x - 1, y)),
                world.get((x + 1, y)),
                world.get((x, y - 1)),
                world.get((x, y + 1)),
                world.get((x + 1, y + 1)),
                world.get((x - 1, y + 1)),
                world.get((x + 1, y - 1)),
                world.get((x - 1, y - 1)),
            ),
            Tile::new(true)
        )
    }

    #[test]
    fn tile_rule_1_with_2_neighbour() {
        /*
         * 010
         * 010  (1,1) == on --> (1,1) == on
         * 010
         */
        let world = World::from_data(
            3,
            vec![
                Tile::new(false),
                Tile::new(true),
                Tile::new(false),
                Tile::new(false),
                Tile::new(true),
                Tile::new(false),
                Tile::new(false),
                Tile::new(true),
                Tile::new(false),
            ],
        );
        let (x, y) = (1, 1);
        assert_eq!(
            world.get((x, y)).unwrap().evolve(
                world.get((x - 1, y)),
                world.get((x + 1, y)),
                world.get((x, y - 1)),
                world.get((x, y + 1)),
                world.get((x + 1, y + 1)),
                world.get((x - 1, y + 1)),
                world.get((x + 1, y - 1)),
                world.get((x - 1, y - 1)),
            ),
            Tile::new(true)
        )
    }

    #[test]
    fn tile_rule_2() {
        /*
         * 010
         * 001  (1,1) == off --> (1,1) == on
         * 010
         */
        let world = World::from_data(
            3,
            vec![
                Tile::new(false),
                Tile::new(true),
                Tile::new(false),
                Tile::new(false),
                Tile::new(false),
                Tile::new(true),
                Tile::new(false),
                Tile::new(true),
                Tile::new(false),
            ],
        );
        let (x, y) = (1, 1);
        assert_eq!(
            world.get((x, y)).unwrap().evolve(
                world.get((x - 1, y)),
                world.get((x + 1, y)),
                world.get((x, y - 1)),
                world.get((x, y + 1)),
                world.get((x + 1, y + 1)),
                world.get((x - 1, y + 1)),
                world.get((x + 1, y - 1)),
                world.get((x - 1, y - 1)),
            ),
            Tile::new(true)
        )
    }

    #[test]
    fn tile_rule_3_alive_to_dead() {
        /*
         * 000
         * 010  (1,1) == on --> (1,1) == off
         * 000
         */
        let world = World::from_data(
            3,
            vec![
                Tile::new(false),
                Tile::new(false),
                Tile::new(false),
                Tile::new(false),
                Tile::new(false),
                Tile::new(false),
                Tile::new(false),
                Tile::new(false),
                Tile::new(false),
            ],
        );
        let (x, y) = (1, 1);
        assert_eq!(
            world.get((x, y)).unwrap().evolve(
                world.get((x - 1, y)),
                world.get((x + 1, y)),
                world.get((x, y - 1)),
                world.get((x, y + 1)),
                world.get((x + 1, y + 1)),
                world.get((x - 1, y + 1)),
                world.get((x + 1, y - 1)),
                world.get((x - 1, y - 1)),
            ),
            Tile::new(false)
        )
    }

    #[test]
    fn tile_rule_3_dead_to_dead() {
        /*
         * 000
         * 000  (1,1) == off --> (1,1) == off
         * 000
         */
        let world = World::from_data(
            3,
            vec![
                Tile::new(false),
                Tile::new(false),
                Tile::new(false),
                Tile::new(false),
                Tile::new(false),
                Tile::new(false),
                Tile::new(false),
                Tile::new(false),
                Tile::new(false),
            ],
        );
        let (x, y) = (1, 1);
        assert_eq!(
            world.get((x, y)).unwrap().evolve(
                world.get((x - 1, y)),
                world.get((x + 1, y)),
                world.get((x, y - 1)),
                world.get((x, y + 1)),
                world.get((x + 1, y + 1)),
                world.get((x - 1, y + 1)),
                world.get((x + 1, y - 1)),
                world.get((x - 1, y - 1)),
            ),
            Tile::new(false)
        )
    }

    #[test]
    fn tile_test_corners() {
        /*
         * 010
         * 110  (0,0) == off --> (0,0) == off
         * 000
         */
        let world = World::from_data(
            3,
            vec![
                Tile::new(false),
                Tile::new(true),
                Tile::new(false),
                Tile::new(true),
                Tile::new(true),
                Tile::new(false),
                Tile::new(false),
                Tile::new(false),
                Tile::new(false),
            ],
        );
        let (x, y) = (1, 1);
        assert_eq!(
            world.get((x, y)).unwrap().evolve(
                world.get((x - 1, y)),
                world.get((x + 1, y)),
                world.get((x, y - 1)),
                world.get((x, y + 1)),
                world.get((x + 1, y + 1)),
                world.get((x - 1, y + 1)),
                world.get((x + 1, y - 1)),
                world.get((x - 1, y - 1)),
            ),
            Tile::new(true)
        )
    }

    #[test]
    fn test_tick_1() {
        /*
         * 000      000
         * 000  ==> 000
         * 000      000
         */

        let seed = vec![
            Tile::new(false),
            Tile::new(false),
            Tile::new(false),
            Tile::new(false),
            Tile::new(false),
            Tile::new(false),
            Tile::new(false),
            Tile::new(false),
            Tile::new(false),
        ];

        let mut world = World::from_data(3, seed.clone());
        world.tick();
        world.tick();

        assert_eq!(world.tiles, seed)
    }

    #[test]
    fn test_tick_2() {
        /*
         * 010      111
         * 111  ==> 101
         * 010      111
         */

        let seed = vec![
            Tile::new(false),
            Tile::new(true),
            Tile::new(false),
            Tile::new(true),
            Tile::new(true),
            Tile::new(true),
            Tile::new(false),
            Tile::new(true),
            Tile::new(false),
        ];

        let tick_one = vec![
            Tile::new(true),
            Tile::new(true),
            Tile::new(true),
            Tile::new(true),
            Tile::new(false),
            Tile::new(true),
            Tile::new(true),
            Tile::new(true),
            Tile::new(true),
        ];

        let mut world = World::from_data(3, seed.clone());
        world.tick();

        assert_eq!(world.tiles, tick_one)
    }
}
