use frigo_core::*;

const TERIYAKI: &str = include_str!("fixtures/teriyaki.json");
const CORBA: &str = include_str!("fixtures/corba.json");
const FILTER: &str = include_str!("fixtures/filter_chicken.json");

fn fridge(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

#[test]
fn parses_a_full_meal() {
    let meals = parse_meals(TERIYAKI).unwrap();
    assert_eq!(meals.len(), 1);
    let meal = &meals[0];
    assert_eq!(meal.id, "52772");
    assert_eq!(meal.name, "Teriyaki Chicken Casserole");
    assert_eq!(meal.ingredients.len(), 9);
    assert_eq!(meal.ingredients[0].name, "soy sauce");
    assert_eq!(meal.ingredients[0].measure, "3/4 cup");
    assert_eq!(meal.tags, ["Meat", "Casserole"]);
    assert!(!meal.instructions.contains('\r'));
}

#[test]
fn parses_summaries_and_empty_answers() {
    let list = parse_summaries(FILTER).unwrap();
    assert_eq!(list.len(), 5);
    assert!(!list[0].id.is_empty());
    assert!(parse_summaries(r#"{"meals":null}"#).unwrap().is_empty());
    assert!(
        parse_summaries(r#"{"meals":"no data found"}"#)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn french_names_become_english() {
    assert_eq!(canonical("Poulet"), "chicken");
    assert_eq!(canonical("œufs"), "egg");
    assert_eq!(api_name("blanc de poulet"), "chicken_breast");
    assert!(same_ingredient("riz", "brown rice"));
    assert!(same_ingredient("sauce soja", "Soy Sauce"));
    assert!(!same_ingredient("egg", "eggplant"));
}

#[test]
fn lists_are_split_and_deduplicated() {
    let list = parse_list("# mon frigo\npoulet, riz; Riz\n- oignons\n\n");
    assert_eq!(list, ["poulet", "riz", "oignons"]);
}

#[test]
fn evaluates_what_is_missing() {
    let meal = &parse_meals(TERIYAKI).unwrap()[0];
    let m = evaluate(
        meal,
        &fridge(&["poulet", "riz", "sauce soja", "ail"]),
        &Criteria::default(),
    );
    assert!(m.have.contains(&"chicken breasts".to_string()));
    assert!(m.have.contains(&"brown rice".to_string()));
    assert!(
        m.have.contains(&"water".to_string()),
        "l'eau est au placard"
    );
    assert!(m.missing.contains(&"cornstarch".to_string()));
    assert_eq!(m.used.len(), 4);
    assert!(m.coverage() > 0.5);
}

#[test]
fn ranks_by_missing_then_time() {
    let mut meals = parse_meals(TERIYAKI).unwrap();
    meals.extend(parse_meals(CORBA).unwrap());
    let ranked = rank(
        &meals,
        &fridge(&["oignon", "carotte", "lentilles"]),
        &Criteria::default(),
    );
    assert_eq!(ranked[0].meal.name, "Corba");
    // Le teriyaki n'utilise rien de ce frigo : il n'est pas proposé.
    assert_eq!(ranked.len(), 1);

    let strict = Criteria {
        strict: true,
        ..Criteria::default()
    };
    assert!(rank(&meals, &fridge(&["oignon"]), &strict).is_empty());
}

#[test]
fn corba_takes_about_half_an_hour() {
    let meal = &parse_meals(CORBA).unwrap()[0];
    let minutes = estimate_minutes(&meal.instructions, meal.ingredients.len());
    assert!((30..=60).contains(&minutes), "{minutes}");
}
