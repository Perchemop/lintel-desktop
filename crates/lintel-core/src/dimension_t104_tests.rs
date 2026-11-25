// T-104: Backward-compatible JSON load tests (AC-004, AC-005)

#[test]
fn a_dimension_without_anchor_fields_in_json_loads_with_none_anchors() {
    // AC-005: v0.12.x fixture file behavior
    // When anchor fields are missing, they should deserialize as None
    let json = r#"{
            "id": 1,
            "kind": "LinearHorizontal",
            "start": { "x": 0.0, "y": 0.0 },
            "end": { "x": 3000.0, "y": 0.0 },
            "offset": 500.0,
            "extension_lines": true,
            "witness_gap": 200.0,
            "witness_overrun": 100.0,
            "label_precision": 0
        }"#;
    let dimension: Dimension = serde_json::from_str(json).expect("dimension loads");
    assert!(
        dimension.start_anchor.is_none(),
        "missing start_anchor should be None"
    );
    assert!(
        dimension.end_anchor.is_none(),
        "missing end_anchor should be None"
    );
    // AC-005: coordinates are preserved unchanged
    assert_eq!(dimension.start, Point2::new(0.0, 0.0));
    assert_eq!(dimension.end, Point2::new(3000.0, 0.0));
}

#[test]
fn anchored_dimension_round_trips_with_constraints_correctly() {
    // AC-004: Save/load project with anchors + constraints → identical
    
    let mut dimension = Dimension::new(
        DimensionId(1),
        DimensionKind::LinearHorizontal,
        Point2::new(0.0, 0.0),
        Point2::new(3000.0, 0.0),
        500.0,
    );
    dimension.start_anchor = Some(DimensionAnchor::anchored(WallId(2), 0.25, false));
    dimension.end_anchor = Some(DimensionAnchor::anchored(WallId(2), 0.75, false));

    // Serialize the dimension
    let json = serde_json::to_string(&dimension).expect("serialize");

    // Deserialize it back
    let back: Dimension = serde_json::from_str(&json).expect("deserialize");

    // Verify exact match
    assert_eq!(back, dimension, "anchored dimension should round-trip exactly");
    assert_eq!(back.start_anchor, Some(DimensionAnchor::anchored(WallId(2), 0.25, false)));
    assert_eq!(back.end_anchor, Some(DimensionAnchor::anchored(WallId(2), 0.75, false)));
}