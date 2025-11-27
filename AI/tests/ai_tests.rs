use ai::*;
use std::collections::HashMap;

#[test]
fn world_state_basics() {
    let mut s = WorldState::new();
    s.insert("Flag".to_string(), FactValue::Bool(true));
    s.insert("Count".to_string(), FactValue::Int(3));
    s.insert("Place".to_string(), FactValue::Str("X".to_string()));

    assert_eq!(s.get("Flag"), Some(&FactValue::Bool(true)));
    assert!(s.satisfies("Place", &FactValue::Str("X".to_string())));

    s.apply_effects(&[("Place".to_string(), FactValue::Str("Y".to_string()))]);
    assert_eq!(s.get("Place"), Some(&FactValue::Str("Y".to_string())));
}

#[test]
fn action_template_and_instance_applicability() {
    let tmpl = move_template(HexCoord { q: 0, r: 0 }, HexCoord { q: 1, r: 0 }, 1.0);
    let mut start = WorldState::new();
    start.insert("At".to_string(), FactValue::Hex(HexCoord { q: 0, r: 0 }));

    assert!(tmpl.is_applicable(&start));

    let inst = ground_action_from_template(&tmpl, Some("agent42".to_string()));
    assert!(inst.is_applicable(&start));
    assert_eq!(inst.agent, Some("agent42".to_string()));
}

#[test]
fn planner_basic_and_limit() {
    // A -> B (cost 2)
    // A -> C (cost 1), C -> B (cost 1) ; optimal A->C->B cost 2
    let mut start = WorldState::new();
    start.insert("At".to_string(), FactValue::Hex(HexCoord { q: 0, r: 0 }));

    let t1 = move_template(HexCoord { q: 0, r: 0 }, HexCoord { q: 2, r: 0 }, 2.0);
    let t2 = move_template(HexCoord { q: 0, r: 0 }, HexCoord { q: 1, r: 0 }, 1.0);
    let t3 = move_template(HexCoord { q: 1, r: 0 }, HexCoord { q: 2, r: 0 }, 1.0);

    let actions = vec![t1, t2, t3];
    let goal = Goal {
        key: "At".to_string(),
        value: FactValue::Hex(HexCoord { q: 2, r: 0 }),
    };

    let plan_opt = plan(&start, &actions, &goal, 1000);
    assert!(plan_opt.is_some());
    let plan_seq = plan_opt.unwrap();
    // Planner may pick the direct A->B (index 0) or the two-step A->C->B (indices 1,2)
    assert!(plan_seq.len() == 1 || plan_seq.len() == 2);

    // Apply chosen plan and verify final world reaches B and cost is <= 2.0
    let instances: Vec<ai::ActionInstance> = actions
        .iter()
        .map(|t| ground_action_from_template(t, None))
        .collect();
    let mut s2 = start.clone();
    let mut total_cost = 0.0f32;
    for &i in &plan_seq {
        let a = &instances[i];
        s2.apply_effects(&a.effects);
        total_cost += a.cost;
    }
    assert_eq!(s2.get("At"), Some(&FactValue::Hex(HexCoord { q: 2, r: 0 })));
    assert!(total_cost <= 2.0 + 1e-6);

    // Too small node budget -> no plan
    let plan_limited = plan(&start, &actions, &goal, 1);
    assert!(plan_limited.is_none());
}

#[test]
fn plan_for_team_sequential_application() {
    // Two agents: a1 moves from X->Y and sets Flag, then a2 depends on Flag
    let mut start = WorldState::new();
    start.insert("At".to_string(), FactValue::Hex(HexCoord { q: 0, r: 0 }));
    start.insert("Flag".to_string(), FactValue::Bool(false));

    // Action visible to a1: move X->Y and set Flag true
    let a1_move = ActionInstance {
        name: "MoveA".to_string(),
        preconditions: vec![("At".to_string(), FactValue::Hex(HexCoord { q: 0, r: 0 }))],
        effects: vec![
            ("At".to_string(), FactValue::Hex(HexCoord { q: 1, r: 0 })),
            ("Flag".to_string(), FactValue::Bool(true)),
        ],
        cost: 1.0,
        agent: Some("a1".to_string()),
    };

    // Action visible to a2: requires Flag==true, sets Done=true
    let a2_action = ActionInstance {
        name: "DoB".to_string(),
        preconditions: vec![("Flag".to_string(), FactValue::Bool(true))],
        effects: vec![("Done".to_string(), FactValue::Bool(true))],
        cost: 1.0,
        agent: Some("a2".to_string()),
    };

    let actions = vec![a1_move.clone(), a2_action.clone()];

    let mut goals: HashMap<String, Vec<Goal>> = HashMap::new();
    goals.insert(
        "a1".to_string(),
        vec![Goal {
            key: "Flag".to_string(),
            value: FactValue::Bool(true),
        }],
    );
    goals.insert(
        "a2".to_string(),
        vec![Goal {
            key: "Done".to_string(),
            value: FactValue::Bool(true),
        }],
    );

    let order = vec!["a1".to_string(), "a2".to_string()];
    let result = plan_for_team(&start, &actions, &goals, &order, 1000);

    // Both agents should have plans; a1's plan should include its action (index 0)
    assert!(result.contains_key("a1"));
    assert!(result.contains_key("a2"));
    assert_eq!(result.get("a1").unwrap(), &vec![0usize]);
    // For a2 the index refers to filtered actions (only a2 visible) -> its plan should be [0]
    assert_eq!(result.get("a2").unwrap(), &vec![0usize]);
}

#[test]
fn executor_instant_and_timed_and_abort() {
    let mut world = WorldState::new();
    world.insert("X".to_string(), FactValue::Int(10));

    // Instant action subtracts 5
    let inst = ActionInstance {
        name: "InstantSub".to_string(),
        preconditions: vec![],
        effects: vec![("X".to_string(), FactValue::Int(5))],
        cost: 0.0,
        agent: None,
    };

    let mut exec = ActionExecutor::new();
    exec.start(RuntimeAction::Instant(inst));
    // update should apply immediately
    let completed = exec.update(0.0, &mut world);
    assert!(completed);
    assert_eq!(world.get("X"), Some(&FactValue::Int(5)));

    // Timed action increases X by 2 when completed
    let timed = ActionInstance {
        name: "TimedAdd".to_string(),
        preconditions: vec![],
        effects: vec![("X".to_string(), FactValue::Int(7))],
        cost: 0.0,
        agent: None,
    };

    exec.start(RuntimeAction::Timed {
        instance: timed,
        duration: 1.0,
        elapsed: 0.0,
    });

    // partial update should not complete
    assert!(!exec.update(0.4, &mut world));
    // still unchanged
    assert_eq!(world.get("X"), Some(&FactValue::Int(5)));

    // complete
    assert!(exec.update(0.6, &mut world));
    assert_eq!(world.get("X"), Some(&FactValue::Int(7)));

    // Start another timed but abort before completion
    let will_abort = ActionInstance {
        name: "AbortMe".to_string(),
        preconditions: vec![],
        effects: vec![("Y".to_string(), FactValue::Bool(true))],
        cost: 0.0,
        agent: None,
    };
    exec.start(RuntimeAction::Timed {
        instance: will_abort,
        duration: 1.0,
        elapsed: 0.0,
    });
    exec.abort();
    // update after abort should be a no-op
    assert!(!exec.update(1.0, &mut world));
    assert!(world.get("Y").is_none());
}

#[test]
fn executor_callbacks_fire() {
    let mut world = WorldState::new();
    world.insert("At".to_string(), FactValue::Hex(HexCoord { q: 0, r: 0 }));

    let inst = ActionInstance {
        name: "InstantMove".to_string(),
        preconditions: vec![],
        effects: vec![("At".to_string(), FactValue::Hex(HexCoord { q: 1, r: 0 }))],
        cost: 0.0,
        agent: None,
    };

    let mut exec = ActionExecutor::new();

    use std::cell::RefCell;
    use std::rc::Rc;

    let started = Rc::new(RefCell::new(0));
    let completed = Rc::new(RefCell::new(0));

    {
        let s = started.clone();
        exec.set_on_start(move |_ai| {
            *s.borrow_mut() += 1;
        });
    }
    {
        let c = completed.clone();
        exec.set_on_complete(move |_ai| {
            *c.borrow_mut() += 1;
        });
    }

    exec.start(RuntimeAction::Instant(inst));
    assert_eq!(*started.borrow(), 1);
    assert!(exec.update(0.0, &mut world));
    assert_eq!(*completed.borrow(), 1);
    // world At should have been updated
    assert_eq!(
        world.get("At"),
        Some(&FactValue::Hex(HexCoord { q: 1, r: 0 }))
    );
}

#[test]
fn attack_template_various_cases() {
    let attack = AttackTemplate {
        name_base: "Atk".to_string(),
        damage: 5,
        cost: 1.0,
        range: 1,
    };

    // Case 1: global enemy keys
    let mut s1 = WorldState::new();
    s1.insert(
        "EnemyAt".to_string(),
        FactValue::Hex(HexCoord { q: 1, r: 0 }),
    );
    s1.insert("EnemyHealth".to_string(), FactValue::Int(6));
    s1.insert("EnemyAlive".to_string(), FactValue::Bool(true));

    let instances = attack.ground_for_state(&s1, Some("ag".to_string()));
    assert_eq!(instances.len(), 1);
    let ai = &instances[0];
    // preconds require At==B (hex) and EnemyAlive==true
    assert!(ai
        .preconditions
        .contains(&("At".to_string(), FactValue::Hex(HexCoord { q: 1, r: 0 }))));
    assert!(ai
        .preconditions
        .contains(&("EnemyAlive".to_string(), FactValue::Bool(true))));
    // effects reduce health from 6 -> 1 and not yet set alive false
    assert!(ai
        .effects
        .contains(&("EnemyHealth".to_string(), FactValue::Int(1))));

    // Case 2: per-id enemy keys without health but with alive flag
    let mut s2 = WorldState::new();
    s2.insert(
        "EnemyAt:orc".to_string(),
        FactValue::Hex(HexCoord { q: 3, r: -1 }),
    );
    s2.insert("EnemyAlive:orc".to_string(), FactValue::Bool(true));

    let insts2 = attack.ground_for_state(&s2, None);
    assert_eq!(insts2.len(), 1);
    let ai2 = &insts2[0];
    // preconds include EnemyAlive:orc==true
    assert!(ai2
        .preconditions
        .contains(&("EnemyAlive:orc".to_string(), FactValue::Bool(true))));
    // effects should set EnemyAlive:orc=false (no health present)
    assert!(ai2
        .effects
        .contains(&("EnemyAlive:orc".to_string(), FactValue::Bool(false))));
}
