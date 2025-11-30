use units::UnitFactory;

#[test]
fn test_name() {
    let unit = UnitFactory::create("Human Noble", None, None).expect("Failed to create unit");
    assert_eq!(unit.name(), "Human Noble");
}

#[test]
fn test_position() {
    let unit = UnitFactory::create(
        "Elf Archer",
        Some("Legolas".to_string()),
        Some(graphics::HexCoord::new(2, 3)),
    )
    .expect("Failed to create unit");
    assert_eq!(unit.position(), graphics::HexCoord::new(2, 3));
}
