import Foundation

/// Comment choisir les recettes.
public struct Criteria: Hashable, Sendable, Codable {
    /// Seulement les recettes faisables avec ce que j'ai.
    public var strict = false
    /// Le sel, le poivre, l'huile, l'eau et le sucre sont considérés comme disponibles.
    public var pantry = true
    /// Au plus ce nombre d'ingrédients manquants (ignoré avec `strict`).
    public var maxMissing: Int?
    /// Au plus ce nombre de minutes (estimées).
    public var maxMinutes: Int?

    public init(strict: Bool = false, pantry: Bool = true, maxMissing: Int? = nil, maxMinutes: Int? = nil) {
        self.strict = strict
        self.pantry = pantry
        self.maxMissing = maxMissing
        self.maxMinutes = maxMinutes
    }
}

/// Une recette, comparée au contenu du frigo.
public struct Match: Identifiable, Hashable, Sendable {
    public var meal: Meal
    public var have: [String]
    public var missing: [String]
    public var used: [String]
    public var minutes: Int

    public var id: String { meal.id }

    /// La part des ingrédients de la recette que j'ai, de 0 à 1.
    public var coverage: Double {
        let total = have.count + missing.count
        return total == 0 ? 0 : Double(have.count) / Double(total)
    }

    /// Faisable tout de suite.
    public var isComplete: Bool { missing.isEmpty }
}

/// Compare une recette au frigo.
public func evaluate(_ meal: Meal, fridge: [String], criteria: Criteria = Criteria()) -> Match {
    var have: [String] = []
    var missing: [String] = []
    var used: [String] = []
    for ingredient in meal.ingredients {
        if let mine = fridge.first(where: { Names.same($0, ingredient.name) }) {
            have.append(ingredient.name)
            if !used.contains(mine) { used.append(mine) }
        } else if criteria.pantry, Names.isPantry(ingredient.name) {
            have.append(ingredient.name)
        } else {
            missing.append(ingredient.name)
        }
    }
    return Match(
        meal: meal,
        have: have,
        missing: missing,
        used: used,
        minutes: estimateMinutes(meal.instructions, ingredients: meal.ingredients.count)
    )
}

/// Les recettes qui utilisent au moins un ingrédient du frigo, de la meilleure à la moins
/// bonne : le moins de manquants, puis le frigo le mieux utilisé, puis la plus rapide.
public func rank(_ meals: [Meal], fridge: [String], criteria: Criteria = Criteria()) -> [Match] {
    var seen = Set<String>()
    var matches: [Match] = []
    for meal in meals where seen.insert(meal.id).inserted {
        let m = evaluate(meal, fridge: fridge, criteria: criteria)
        if m.used.isEmpty { continue }
        if criteria.strict, !m.isComplete { continue }
        if !criteria.strict, let max = criteria.maxMissing, m.missing.count > max { continue }
        if let max = criteria.maxMinutes, m.minutes > max { continue }
        matches.append(m)
    }
    return matches.sorted(by: isBetter)
}

/// L'ordre du classement.
func isBetter(_ a: Match, than b: Match) -> Bool {
    if a.missing.count != b.missing.count { return a.missing.count < b.missing.count }
    if a.used.count != b.used.count { return a.used.count > b.used.count }
    if a.minutes != b.minutes { return a.minutes < b.minutes }
    return a.meal.name < b.meal.name
}

/// Le temps estimé, en minutes : les durées écrites dans les instructions, plus environ une
/// minute et demie par ingrédient. Sans durée écrite, cinq minutes par étape.
public func estimateMinutes(_ instructions: String, ingredients: Int) -> Int {
    let prep = (ingredients * 3 + 1) / 2
    let written = writtenMinutes(instructions)
    let cooking: Int
    if written > 0 {
        cooking = written
    } else {
        let steps = instructions
            .split(whereSeparator: { $0 == "\n" || $0 == "." })
            .filter { $0.trimmingCharacters(in: .whitespaces).count > 12 }
            .count
        cooking = max(steps, 1) * 5
    }
    return min(max(cooking + prep, 5), 24 * 60)
}

/// La somme des durées écrites : « 15-20 minutes » compte 20, « 1 1/2 hours » compte 90.
func writtenMinutes(_ text: String) -> Int {
    let separators = CharacterSet.whitespacesAndNewlines.union(CharacterSet(charactersIn: ",;():"))
    let words = text.lowercased().components(separatedBy: separators).filter { !$0.isEmpty }
    var total = 0.0
    for (i, raw) in words.enumerated() {
        let word = raw.trimmingCharacters(in: CharacterSet(charactersIn: ".!"))
        let unit: Double
        if word.hasPrefix("min") {
            unit = 1
        } else if word.hasPrefix("hour") || ["hr", "hrs", "h"].contains(word) {
            unit = 60
        } else {
            continue
        }
        var fraction = 0.0
        var value: Double?
        var j = i
        while j > 0, i - j < 4 {
            j -= 1
            let w = words[j]
            guard let n = number(w) else { break }
            if w.contains("/") || ["½", "¼", "¾"].contains(w) {
                fraction = n
            } else {
                value = n
                break
            }
        }
        let amount = (value ?? 0) + fraction
        if amount > 0 { total += amount * unit }
    }
    return Int(total.rounded())
}

func number(_ word: String) -> Double? {
    if let dash = word.firstIndex(where: { $0 == "-" || $0 == "–" }) {
        let (a, b) = (String(word[..<dash]), String(word[word.index(after: dash)...]))
        return number(b) ?? number(a)
    }
    if let slash = word.firstIndex(of: "/") {
        guard let a = Double(word[..<slash]), let b = Double(word[word.index(after: slash)...]), b > 0 else { return nil }
        return a / b
    }
    switch word {
    case "½": return 0.5
    case "¼": return 0.25
    case "¾": return 0.75
    case "a", "an", "one": return 1
    case "two": return 2
    case "few": return 3
    default:
        guard let n = Double(word), n.isFinite, n >= 0, n < 1000 else { return nil }
        return n
    }
}
