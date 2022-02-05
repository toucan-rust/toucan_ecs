use components::{Mass, Position};
use toucan_ecs::Registry;

mod components;

#[test]
fn extend() {
    let mut registry = Registry::new();

    let entities = registry.extend(1_000).to_vec();
    assert!(!registry.is_empty());
    assert_eq!(entities.len(), 1_000);

    for entity in entities {
        assert!(registry.is_entity_empty(entity));
    }
}

#[test]
fn extend_with_one() {
    let mut registry = Registry::new();

    let entities = registry.extend_with_one([Mass(0.0); 1_000]).to_vec();
    assert!(!registry.is_empty());
    assert_eq!(entities.len(), 1_000);

    for entity in entities {
        assert!(!registry.is_entity_empty(entity));
        assert!(registry.attached_one::<Mass>(entity));
    }
}

#[test]
fn extend_with() {
    let mut registry = Registry::new();

    let entities = registry
        .extend_with(std::iter::repeat((Mass(1.0), Position { x: 0.0, y: 0.0 })).take(1_000))
        .to_vec();
    assert!(!registry.is_empty());
    assert_eq!(entities.len(), 1_000);

    for entity in entities {
        assert!(!registry.is_entity_empty(entity));
        assert!(registry.attached::<(Position, Mass)>(entity));
    }
}
