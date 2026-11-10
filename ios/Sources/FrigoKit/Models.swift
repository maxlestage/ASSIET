import Foundation

/// Un ingrédient d'une recette, avec sa quantité (« 2 cups », « 1 pinch »…).
public struct Ingredient: Hashable, Sendable, Codable {
    public var name: String
    public var measure: String

    public init(name: String, measure: String = "") {
        self.name = name
        self.measure = measure
    }
}

/// Une recette complète (réponse de `lookup.php`).
public struct Meal: Identifiable, Hashable, Sendable, Codable {
    public var id: String
    public var name: String
    public var category: String?
    public var area: String?
    public var instructions: String
    public var thumbnail: URL?
    public var youtube: URL?
    public var tags: [String]
    public var ingredients: [Ingredient]

    public init(
        id: String, name: String, category: String? = nil, area: String? = nil,
        instructions: String = "", thumbnail: URL? = nil, youtube: URL? = nil,
        tags: [String] = [], ingredients: [Ingredient] = []
    ) {
        self.id = id
        self.name = name
        self.category = category
        self.area = area
        self.instructions = instructions
        self.thumbnail = thumbnail
        self.youtube = youtube
        self.tags = tags
        self.ingredients = ingredients
    }

    /// Les étapes : une par ligne non vide, sans les « STEP 1 » que certaines recettes ont.
    public var steps: [String] {
        instructions
            .split(whereSeparator: \.isNewline)
            .map { $0.trimmingCharacters(in: .whitespaces) }
            .filter { line in
                let lower = line.lowercased()
                let header = lower.hasPrefix("step") && lower.count <= 8
                let number = !line.isEmpty && line.allSatisfy { $0.isNumber || $0 == "." }
                return !line.isEmpty && !header && !number
            }
    }
}

/// Le résumé d'une recette (réponse de `filter.php`).
public struct MealSummary: Identifiable, Hashable, Sendable, Decodable {
    public var id: String
    public var name: String
    public var thumbnail: URL?

    enum CodingKeys: String, CodingKey {
        case id = "idMeal"
        case name = "strMeal"
        case thumbnail = "strMealThumb"
    }
}

/// Une clé quelconque : les champs `strIngredient1` … `strIngredient20` sont lus un à un.
struct DynamicKey: CodingKey {
    var stringValue: String
    var intValue: Int? { nil }
    init(_ string: String) { stringValue = string }
    init?(stringValue: String) { self.stringValue = stringValue }
    init?(intValue: Int) { nil }
}

/// Une recette brute de TheMealDB.
struct RawMeal: Decodable {
    let meal: Meal?

    init(from decoder: any Decoder) throws {
        let c = try decoder.container(keyedBy: DynamicKey.self)
        func text(_ key: String) -> String? {
            guard let value = try? c.decodeIfPresent(String.self, forKey: DynamicKey(key)) else { return nil }
            let trimmed = value.trimmingCharacters(in: .whitespacesAndNewlines)
            return trimmed.isEmpty ? nil : trimmed
        }
        guard let id = text("idMeal"), let name = text("strMeal") else {
            meal = nil
            return
        }
        var ingredients: [Ingredient] = []
        var i = 1
        while c.contains(DynamicKey("strIngredient\(i)")) {
            if let ingredient = text("strIngredient\(i)") {
                ingredients.append(Ingredient(name: ingredient, measure: text("strMeasure\(i)") ?? ""))
            }
            i += 1
        }
        meal = Meal(
            id: id,
            name: name,
            category: text("strCategory"),
            area: text("strArea") ?? text("strCountry"),
            instructions: (text("strInstructions") ?? "").replacingOccurrences(of: "\r\n", with: "\n"),
            thumbnail: text("strMealThumb").flatMap(URL.init(string:)),
            youtube: text("strYoutube").flatMap(URL.init(string:)),
            tags: (text("strTags") ?? "")
                .split(separator: ",")
                .map { $0.trimmingCharacters(in: .whitespaces) }
                .filter { !$0.isEmpty },
            ingredients: ingredients
        )
    }
}

/// `{"meals": [...]}`, `{"meals": null}` ou `{"meals": "no data found"}`.
struct Envelope<T: Decodable>: Decodable {
    let meals: [T]

    enum CodingKeys: String, CodingKey { case meals }

    init(from decoder: any Decoder) throws {
        let c = try decoder.container(keyedBy: CodingKeys.self)
        meals = (try? c.decodeIfPresent([T].self, forKey: .meals)) ?? []
    }
}

/// Lit une réponse de `lookup.php` / `search.php`.
public func parseMeals(_ data: Data) throws -> [Meal] {
    try JSONDecoder().decode(Envelope<RawMeal>.self, from: data).meals.compactMap(\.meal)
}

/// Lit une réponse de `filter.php`.
public func parseSummaries(_ data: Data) throws -> [MealSummary] {
    try JSONDecoder().decode(Envelope<MealSummary>.self, from: data).meals
}
