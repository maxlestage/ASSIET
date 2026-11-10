//! Les noms d'ingrédients : minuscules, sans accents, au singulier, traduits du
//! français vers l'anglais de TheMealDB, et comparés mot à mot.

/// Les ingrédients que tout le monde a dans son placard : ils ne comptent pas comme
/// « manquants » (sauf avec `--sans-placard`).
pub const PANTRY: &[&str] = &[
    "water",
    "salt",
    "sea salt",
    "pepper",
    "black pepper",
    "salt and pepper",
    "oil",
    "olive oil",
    "vegetable oil",
    "sunflower oil",
    "sugar",
    "ice",
    "ice cube",
];

/// Français → anglais (TheMealDB est en anglais). Les clés sont déjà normalisées.
const FRENCH: &[(&str, &str)] = &[
    ("poulet", "chicken"),
    ("blanc de poulet", "chicken breast"),
    ("cuisse de poulet", "chicken thigh"),
    ("boeuf", "beef"),
    ("viande hachee", "minced beef"),
    ("steak hache", "minced beef"),
    ("porc", "pork"),
    ("agneau", "lamb"),
    ("veau", "veal"),
    ("canard", "duck"),
    ("dinde", "turkey"),
    ("lardon", "bacon"),
    ("bacon", "bacon"),
    ("jambon", "ham"),
    ("saucisse", "sausage"),
    ("chorizo", "chorizo"),
    ("saumon", "salmon"),
    ("thon", "tuna"),
    ("cabillaud", "cod"),
    ("crevette", "prawn"),
    ("moule", "mussel"),
    ("oeuf", "egg"),
    ("lait", "milk"),
    ("beurre", "butter"),
    ("creme", "double cream"),
    ("creme fraiche", "creme fraiche"),
    ("fromage", "cheese"),
    ("parmesan", "parmesan"),
    ("mozzarella", "mozzarella"),
    ("cheddar", "cheddar cheese"),
    ("feta", "feta"),
    ("yaourt", "yogurt"),
    ("riz", "rice"),
    ("pate", "pasta"),
    ("spaghetti", "spaghetti"),
    ("nouille", "noodle"),
    ("farine", "flour"),
    ("pain", "bread"),
    ("chapelure", "breadcrumb"),
    ("pomme de terre", "potato"),
    ("patate", "potato"),
    ("patate douce", "sweet potato"),
    ("carotte", "carrot"),
    ("oignon", "onion"),
    ("oignon rouge", "red onion"),
    ("echalote", "shallot"),
    ("ail", "garlic"),
    ("tomate", "tomato"),
    ("concentre de tomate", "tomato puree"),
    ("poivron", "red pepper"),
    ("poivron rouge", "red pepper"),
    ("courgette", "courgette"),
    ("aubergine", "aubergine"),
    ("champignon", "mushroom"),
    ("epinard", "spinach"),
    ("brocoli", "broccoli"),
    ("chou", "cabbage"),
    ("chou fleur", "cauliflower"),
    ("haricot vert", "green bean"),
    ("petit pois", "peas"),
    ("lentille", "lentil"),
    ("pois chiche", "chickpea"),
    ("mais", "sweetcorn"),
    ("concombre", "cucumber"),
    ("salade", "lettuce"),
    ("avocat", "avocado"),
    ("citron", "lemon"),
    ("citron vert", "lime"),
    ("pomme", "apple"),
    ("banane", "banana"),
    ("fraise", "strawberry"),
    ("orange", "orange"),
    ("gingembre", "ginger"),
    ("persil", "parsley"),
    ("coriandre", "coriander"),
    ("basilic", "basil"),
    ("thym", "thyme"),
    ("romarin", "rosemary"),
    ("menthe", "mint"),
    ("cumin", "cumin"),
    ("paprika", "paprika"),
    ("cannelle", "cinnamon"),
    ("sucre", "sugar"),
    ("sel", "salt"),
    ("poivre", "black pepper"),
    ("huile", "oil"),
    ("huile d olive", "olive oil"),
    ("vinaigre", "vinegar"),
    ("sauce soja", "soy sauce"),
    ("miel", "honey"),
    ("moutarde", "mustard"),
    ("chocolat", "chocolate"),
    ("noix de coco", "coconut"),
    ("lait de coco", "coconut milk"),
    ("bouillon", "stock"),
    ("vin blanc", "white wine"),
    ("vin rouge", "red wine"),
    ("eau", "water"),
];

/// Les mots qui ne disent rien de l'ingrédient (« chopped tomatoes » = des tomates).
const NOISE: &[&str] = &[
    "fresh", "chopped", "minced", "sliced", "diced", "large", "small", "medium", "ground",
    "grated", "dried", "frozen", "boneless", "skinless", "finely", "roughly", "crushed", "whole",
    "free", "range", "organic", "of", "de", "du", "des", "la", "le", "les", "d",
];

/// Minuscules, sans accents ni ponctuation, mots au singulier, espaces simples.
///
/// ```
/// assert_eq!(frigo_core::normalize("  Œufs, BIO "), "oeuf bio");
/// assert_eq!(frigo_core::normalize("Chicken Breasts"), "chicken breast");
/// ```
pub fn normalize(name: &str) -> String {
    let mut flat = String::with_capacity(name.len());
    for c in name.chars().flat_map(char::to_lowercase) {
        match c {
            'à' | 'â' | 'ä' | 'á' | 'ã' => flat.push('a'),
            'é' | 'è' | 'ê' | 'ë' => flat.push('e'),
            'î' | 'ï' | 'í' => flat.push('i'),
            'ô' | 'ö' | 'ó' => flat.push('o'),
            'û' | 'ü' | 'ù' | 'ú' => flat.push('u'),
            'ç' => flat.push('c'),
            'ñ' => flat.push('n'),
            'œ' => flat.push_str("oe"),
            'æ' => flat.push_str("ae"),
            c if c.is_alphanumeric() => flat.push(c),
            _ => flat.push(' '),
        }
    }
    flat.split_whitespace()
        .map(singular)
        .collect::<Vec<_>>()
        .join(" ")
}

fn singular(word: &str) -> &str {
    // « tomatoes » → « tomato », « peas » et « asparagus » restent comme ils sont.
    const KEEP: &[&str] = &[
        "peas",
        "asparagus",
        "couscous",
        "hummus",
        "swiss",
        "molasses",
    ];
    if word.len() <= 3 || KEEP.contains(&word) || word.ends_with("ss") || word.ends_with("us") {
        return word;
    }
    if let Some(stem) = word.strip_suffix("oes") {
        return &word[..stem.len() + 1];
    }
    if let Some(stem) = word.strip_suffix("ies") {
        // « berries » : on garde « berr », assez pour comparer deux mots normalisés.
        return stem;
    }
    if word.ends_with("ux") {
        // « choux », « poireaux »
        return &word[..word.len() - 1];
    }
    word.strip_suffix('s').unwrap_or(word)
}

/// Le nom normalisé, traduit en anglais quand il est français.
///
/// ```
/// assert_eq!(frigo_core::canonical("Pommes de terre"), "potato");
/// assert_eq!(frigo_core::canonical("cheddar"), "cheddar cheese");
/// assert_eq!(frigo_core::canonical("Tomatoes"), "tomato");
/// ```
pub fn canonical(name: &str) -> String {
    let normalized = normalize(name);
    FRENCH
        .iter()
        .find(|(fr, _)| *fr == normalized)
        .map(|(_, en)| normalize(en))
        .unwrap_or(normalized)
}

/// Le nom à donner à `filter.php?i=` (« chicken breast » → « chicken_breast »).
pub fn api_name(name: &str) -> String {
    canonical(name).replace(' ', "_")
}

fn keywords(name: &str) -> Vec<String> {
    normalize(name)
        .split(' ')
        .filter(|w| !w.is_empty() && !NOISE.contains(w))
        .map(str::to_owned)
        .collect()
}

/// Deux ingrédients sont « les mêmes » quand les mots de l'un sont tous dans l'autre :
/// « chicken » couvre « chicken breasts », « garlic » couvre « minced garlic », mais
/// « egg » ne couvre pas « eggplant ».
///
/// ```
/// use frigo_core::same_ingredient;
/// assert!(same_ingredient("chicken", "Chicken Breasts"));
/// assert!(same_ingredient("Minced Garlic", "ail"));
/// assert!(!same_ingredient("egg", "eggplant"));
/// ```
pub fn same_ingredient(a: &str, b: &str) -> bool {
    let a = keywords(&canonical(a));
    let b = keywords(&canonical(b));
    if a.is_empty() || b.is_empty() {
        return false;
    }
    let contains = |big: &[String], small: &[String]| small.iter().all(|w| big.contains(w));
    contains(&a, &b) || contains(&b, &a)
}

/// Un ingrédient du placard (eau, sel, poivre, huile, sucre…).
pub fn is_pantry(name: &str) -> bool {
    let name = canonical(name);
    PANTRY.iter().any(|p| normalize(p) == name)
}

/// Découpe une liste tapée à la main : virgules, points-virgules ou retours à la ligne ;
/// les lignes qui commencent par `#` sont des commentaires.
pub fn parse_list(text: &str) -> Vec<String> {
    let mut items: Vec<String> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('#') {
            continue;
        }
        for item in line.split([',', ';']) {
            let item = item.trim().trim_start_matches(['-', '*', '•']).trim();
            if !item.is_empty() && !items.iter().any(|i| canonical(i) == canonical(item)) {
                items.push(item.to_owned());
            }
        }
    }
    items
}
