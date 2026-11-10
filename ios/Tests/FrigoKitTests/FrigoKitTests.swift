import Foundation
import Testing
@testable import FrigoKit

private func fixture(_ name: String) throws -> Data {
    let url = try #require(Bundle.module.url(forResource: name, withExtension: "json", subdirectory: "Fixtures"))
    return try Data(contentsOf: url)
}

@Suite("Les réponses de TheMealDB")
struct ParsingTests {
    @Test func parsesAFullMeal() throws {
        let meals = try parseMeals(fixture("teriyaki"))
        let meal = try #require(meals.first)
        #expect(meal.id == "52772")
        #expect(meal.name == "Teriyaki Chicken Casserole")
        #expect(meal.ingredients.count == 9)
        #expect(meal.ingredients[0] == Ingredient(name: "soy sauce", measure: "3/4 cup"))
        #expect(meal.tags == ["Meat", "Casserole"])
        #expect(!meal.instructions.contains("\r"))
    }

    @Test func parsesSummariesAndEmptyAnswers() throws {
        #expect(try parseSummaries(fixture("filter_chicken")).count == 5)
        #expect(try parseSummaries(Data(#"{"meals":null}"#.utf8)).isEmpty)
        #expect(try parseSummaries(Data(#"{"meals":"no data found"}"#.utf8)).isEmpty)
    }
}

@Suite("Les noms d'ingrédients")
struct NamesTests {
    @Test(arguments: [
        ("Poulet", "chicken"),
        ("œufs", "egg"),
        ("Pommes de terre", "potato"),
        ("Tomatoes", "tomato"),
        ("Chicken Breasts", "chicken breast"),
    ])
    func canonical(input: String, expected: String) {
        #expect(Names.canonical(input) == expected)
    }

    @Test func comparesWordByWord() {
        #expect(Names.same("riz", "brown rice"))
        #expect(Names.same("sauce soja", "Soy Sauce"))
        #expect(!Names.same("egg", "eggplant"))
        #expect(Names.apiName("blanc de poulet") == "chicken_breast")
    }

    @Test func splitsLists() {
        #expect(Names.parseList("# mon frigo\npoulet, riz; Riz\n- oignons\n") == ["poulet", "riz", "oignons"])
    }
}

@Suite("Le classement")
struct RankingTests {
    @Test func evaluatesWhatIsMissing() throws {
        let meal = try #require(try parseMeals(fixture("teriyaki")).first)
        let m = evaluate(meal, fridge: ["poulet", "riz", "sauce soja", "ail"])
        #expect(m.have.contains("chicken breasts"))
        #expect(m.have.contains("water"))
        #expect(m.missing.contains("cornstarch"))
        #expect(m.used.count == 4)
        #expect(m.coverage > 0.5)
    }

    @Test func ranksAndFilters() throws {
        let meals = try parseMeals(fixture("teriyaki")) + parseMeals(fixture("corba"))
        let ranked = rank(meals, fridge: ["oignon", "carotte", "lentilles"])
        #expect(ranked.map(\.meal.name) == ["Corba"])
        #expect(rank(meals, fridge: ["oignon"], criteria: Criteria(strict: true)).isEmpty)
    }

    @Test func estimatesTime() {
        #expect(writtenMinutes("cook for 15-20 minutes") == 20)
        #expect(writtenMinutes("bake 1 1/2 hours") == 90)
        #expect(writtenMinutes("rest for an hour") == 60)
        #expect(writtenMinutes("stir for 10 seconds") == 0)
        #expect(estimateMinutes("Fry for 5 minutes.\nBake 1 hour.", ingredients: 4) == 71)
    }
}
