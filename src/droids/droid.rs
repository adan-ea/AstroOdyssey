trait Droid {
    fn new(iron: u32) -> Self where Self: Sized;
    fn move_(&mut self, environment: &Environment);
    fn consume_energy(&mut self, action: Action);
    fn craft_cost() -> u32;
}

// enum of the possible environments
enum Environment {
    Unknown,
    Known,
    Water,
}

// enum of the different possible actions for a droid
enum Action {
    Move(Environment),
    Mine,
    Heal,
    Unload,
    Craft,
}
