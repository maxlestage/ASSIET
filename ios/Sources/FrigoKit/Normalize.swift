import Foundation

/// Les noms d'ingrédients : minuscules, sans accents, au singulier, traduits du français
/// (ou de l'espagnol) vers l'anglais de TheMealDB, et comparés mot à mot. Le même
/// algorithme que `frigo-core` (Rust).
public enum Names {
    /// Ce que tout le monde a au placard : jamais « manquant » (sauf sans placard).
    public static let pantry: Set<String> = [
        "water", "salt", "sea salt", "pepper", "black pepper", "salt and pepper", "oil",
        "olive oil", "vegetable oil", "sunflower oil", "sugar", "ice", "ice cube",
    ]

    /// Français → anglais. Les clés sont déjà normalisées.
    static let french: [String: String] = [
        "poulet": "chicken", "blanc de poulet": "chicken breast", "cuisse de poulet": "chicken thigh",
        "boeuf": "beef", "viande hachee": "minced beef", "steak hache": "minced beef",
        "porc": "pork", "agneau": "lamb", "veau": "veal", "canard": "duck", "dinde": "turkey",
        "lardon": "bacon", "bacon": "bacon", "jambon": "ham", "saucisse": "sausage",
        "chorizo": "chorizo", "saumon": "salmon", "thon": "tuna", "cabillaud": "cod",
        "crevette": "prawn", "moule": "mussel", "oeuf": "egg", "lait": "milk", "beurre": "butter",
        "creme": "double cream", "creme fraiche": "creme fraiche", "fromage": "cheese",
        "parmesan": "parmesan", "mozzarella": "mozzarella", "cheddar": "cheddar cheese",
        "feta": "feta", "yaourt": "yogurt", "riz": "rice", "pate": "pasta",
        "spaghetti": "spaghetti", "nouille": "noodle", "farine": "flour", "pain": "bread",
        "chapelure": "breadcrumb", "pomme de terre": "potato", "patate": "potato",
        "patate douce": "sweet potato", "carotte": "carrot", "oignon": "onion",
        "oignon rouge": "red onion", "echalote": "shallot", "ail": "garlic", "tomate": "tomato",
        "concentre de tomate": "tomato puree", "poivron": "red pepper", "poivron rouge": "red pepper",
        "courgette": "courgette", "aubergine": "aubergine", "champignon": "mushroom",
        "epinard": "spinach", "brocoli": "broccoli", "chou": "cabbage", "chou fleur": "cauliflower",
        "haricot vert": "green bean", "petit pois": "peas", "lentille": "lentil",
        "pois chiche": "chickpea", "mais": "sweetcorn", "concombre": "cucumber",
        "salade": "lettuce", "avocat": "avocado", "citron": "lemon", "citron vert": "lime",
        "pomme": "apple", "banane": "banana", "fraise": "strawberry", "orange": "orange",
        "gingembre": "ginger", "persil": "parsley", "coriandre": "coriander", "basilic": "basil",
        "thym": "thyme", "romarin": "rosemary", "menthe": "mint", "cumin": "cumin",
        "paprika": "paprika", "cannelle": "cinnamon", "sucre": "sugar", "sel": "salt",
        "poivre": "black pepper", "huile": "oil", "huile d olive": "olive oil",
        "vinaigre": "vinegar", "sauce soja": "soy sauce", "miel": "honey", "moutarde": "mustard",
        "chocolat": "chocolate", "noix de coco": "coconut", "lait de coco": "coconut milk",
        "bouillon": "stock", "vin blanc": "white wine", "vin rouge": "red wine", "eau": "water",
    ]

    /// Espagnol → anglais. Les clés sont déjà normalisées : un pluriel en « -es » perd
    /// seulement son « s » (« limones » → « limone »), d'où quelques clés en double. Une clé
    /// qui est aussi un mot anglais de TheMealDB n'est jamais détournée (« pasta »,
    /// « chorizo », « salmon », « cilantro » restent tels quels).
    static let spanish: [String: String] = [
        "pollo": "chicken", "pechuga de pollo": "chicken breast", "muslo de pollo": "chicken thigh",
        "ternera": "beef", "carne de res": "beef", "carne picada": "minced beef",
        "cerdo": "pork", "cordero": "lamb", "pato": "duck", "pavo": "turkey",
        "tocino": "bacon", "beicon": "bacon", "panceta": "bacon", "jamon": "ham",
        "salchicha": "sausage", "atun": "tuna", "bacalao": "cod",
        "gamba": "prawn", "camaron": "prawn", "camarone": "prawn", "langostino": "prawn",
        "mejillon": "mussel", "mejillone": "mussel", "huevo": "egg", "leche": "milk",
        "mantequilla": "butter", "nata": "double cream", "queso": "cheese",
        "parmesano": "parmesan", "queso parmesano": "parmesan", "queso feta": "feta",
        "yogur": "yogurt", "arroz": "rice", "espagueti": "spaghetti", "fideo": "noodle",
        "harina": "flour", "pan": "bread", "pan rallado": "breadcrumb",
        "patata": "potato", "papa": "potato", "batata": "sweet potato", "boniato": "sweet potato",
        "camote": "sweet potato", "zanahoria": "carrot", "cebolla": "onion",
        "cebolla morada": "red onion", "cebolla roja": "red onion", "chalota": "shallot",
        "ajo": "garlic", "tomate": "tomato", "tomate concentrado": "tomato puree",
        "concentrado de tomate": "tomato puree", "pimiento": "red pepper",
        "pimiento rojo": "red pepper", "calabacin": "courgette", "calabacine": "courgette",
        "berenjena": "aubergine", "champinon": "mushroom", "champinone": "mushroom",
        "seta": "mushroom", "espinaca": "spinach", "brocoli": "broccoli", "col": "cabbage",
        "repollo": "cabbage", "coliflor": "cauliflower", "judia verde": "green bean",
        "guisante": "peas", "lenteja": "lentil", "garbanzo": "chickpea", "maiz": "sweetcorn",
        "pepino": "cucumber", "lechuga": "lettuce", "aguacate": "avocado",
        "limon": "lemon", "limone": "lemon", "lima": "lime", "manzana": "apple",
        "platano": "banana", "fresa": "strawberry", "naranja": "orange",
        "jengibre": "ginger", "perejil": "parsley", "albahaca": "basil", "tomillo": "thyme",
        "romero": "rosemary", "menta": "mint", "hierbabuena": "mint", "comino": "cumin",
        "pimenton": "paprika", "canela": "cinnamon", "azucar": "sugar", "sal": "salt",
        "pimienta": "black pepper", "pimienta negra": "black pepper", "aceite": "oil",
        "aceite de oliva": "olive oil", "vinagre": "vinegar", "salsa de soja": "soy sauce",
        "salsa de soya": "soy sauce", "miel": "honey", "mostaza": "mustard",
        "coco": "coconut", "leche de coco": "coconut milk", "caldo": "stock",
        "vino blanco": "white wine", "vino tinto": "red wine", "agua": "water",
    ]

    /// Les mots qui ne disent rien de l'ingrédient (« chopped tomatoes » = des tomates).
    static let noise: Set<String> = [
        "fresh", "chopped", "minced", "sliced", "diced", "large", "small", "medium", "ground",
        "grated", "dried", "frozen", "boneless", "skinless", "finely", "roughly", "crushed",
        "whole", "free", "range", "organic", "of", "de", "du", "des", "la", "le", "les", "d",
        "del", "el", "los", "las",
    ]

    static let keep: Set<String> = ["peas", "asparagus", "couscous", "hummus", "swiss", "molasses"]

    /// Minuscules, sans accents ni ponctuation, mots au singulier, espaces simples.
    public static func normalize(_ name: String) -> String {
        var flat = ""
        for c in name.lowercased() {
            switch c {
            case "œ": flat += "oe"
            case "æ": flat += "ae"
            default:
                let plain = String(c).folding(options: .diacriticInsensitive, locale: Locale(identifier: "fr_FR"))
                if plain.unicodeScalars.allSatisfy({ CharacterSet.alphanumerics.contains($0) }) {
                    flat += plain
                } else {
                    flat += " "
                }
            }
        }
        return flat.split(separator: " ").map { singular(String($0)) }.joined(separator: " ")
    }

    static func singular(_ word: String) -> String {
        if word.count <= 3 || keep.contains(word) || word.hasSuffix("ss") || word.hasSuffix("us") {
            return word
        }
        if word.hasSuffix("oes") { return String(word.dropLast(2)) }
        if word.hasSuffix("ies") { return String(word.dropLast(3)) }
        if word.hasSuffix("ux") { return String(word.dropLast()) }
        if word.hasSuffix("s") { return String(word.dropLast()) }
        return word
    }

    /// Le nom normalisé, traduit en anglais quand il est français ou espagnol.
    public static func canonical(_ name: String) -> String {
        let normalized = normalize(name)
        return (french[normalized] ?? spanish[normalized]).map(normalize) ?? normalized
    }

    /// Le nom à donner à `filter.php?i=` (« chicken breast » → « chicken_breast »).
    public static func apiName(_ name: String) -> String {
        canonical(name).replacingOccurrences(of: " ", with: "_")
    }

    static func keywords(_ name: String) -> [String] {
        normalize(name).split(separator: " ").map(String.init).filter { !noise.contains($0) }
    }

    /// Deux ingrédients sont « les mêmes » quand les mots de l'un sont tous dans l'autre.
    public static func same(_ a: String, _ b: String) -> Bool {
        let a = keywords(canonical(a))
        let b = keywords(canonical(b))
        guard !a.isEmpty, !b.isEmpty else { return false }
        return Set(b).isSubset(of: a) || Set(a).isSubset(of: b)
    }

    /// Un ingrédient du placard.
    public static func isPantry(_ name: String) -> Bool {
        let name = canonical(name)
        return pantry.contains { normalize($0) == name }
    }

    /// Découpe une liste tapée à la main (virgules, points-virgules, lignes, `#` commente).
    public static func parseList(_ text: String) -> [String] {
        var items: [String] = []
        for line in text.split(whereSeparator: \.isNewline) {
            let line = line.trimmingCharacters(in: .whitespaces)
            if line.hasPrefix("#") { continue }
            for raw in line.split(whereSeparator: { $0 == "," || $0 == ";" }) {
                let item = raw.trimmingCharacters(in: .whitespaces)
                    .trimmingCharacters(in: CharacterSet(charactersIn: "-*•"))
                    .trimmingCharacters(in: .whitespaces)
                if !item.isEmpty, !items.contains(where: { canonical($0) == canonical(item) }) {
                    items.append(item)
                }
            }
        }
        return items
    }
}
