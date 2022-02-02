use components::{Mass, Position, Velocity};
use toucan_ecs::Registry;

mod components;

#[test]
fn view_one_mut() {
    let mut registry = Registry::new();

    for i in 0..=10 {
        let f = i as f32;
        let position = Position { x: f, y: f };
        let velocity = Velocity {
            dx: f / 10.0,
            dy: -f / 10.0,
        };
        let mass = Mass(f);
        let entity = registry.create();
        if let Some(mut entry) = registry.entry(entity) {
            entry.attach((position, velocity, mass));
        }
    }

    for (entity, mut component) in registry.view_mut_one::<Position>() {
        component.y -= 10.0;
        println!("entity: {:?}, component: {:?}", entity, *component)
    }
}

#[test]
fn view_mut() {}