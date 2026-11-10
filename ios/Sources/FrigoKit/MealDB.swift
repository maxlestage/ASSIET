import Foundation
#if canImport(FoundationNetworking)
import FoundationNetworking
#endif

/// Le client de TheMealDB (gratuit, clé publique `1`).
///
/// L'API ne cherche que par un ingrédient à la fois : on cherche chaque ingrédient du
/// frigo en parallèle (un `TaskGroup`), on garde les recettes trouvées le plus souvent, on
/// lit leur détail, puis on les classe localement avec ``rank(_:fridge:criteria:)``.
public actor MealDB {
    public static let defaultBaseURL = URL(string: "https://www.themealdb.com/api/json/v1/1")!

    public enum Failure: Error, LocalizedError, Sendable {
        case emptyFridge
        case http(Int)

        public var errorDescription: String? {
            switch self {
            case .emptyFridge: "Le frigo est vide : ajoutez au moins un ingrédient."
            case .http(let status): "TheMealDB a répondu \(status)."
            }
        }
    }

    let baseURL: URL
    let session: URLSession
    /// Le nombre de recettes dont on lit le détail.
    public var maxDetails = 24
    /// Le nombre de requêtes en même temps.
    public var concurrency = 6

    public init(baseURL: URL = MealDB.defaultBaseURL, session: URLSession = .shared) {
        self.baseURL = baseURL
        self.session = session
    }

    func get(_ path: String, _ query: [String: String]) async throws -> Data {
        var components = URLComponents(url: baseURL.appendingPathComponent(path), resolvingAgainstBaseURL: false)!
        components.queryItems = query.map { URLQueryItem(name: $0.key, value: $0.value) }
        var request = URLRequest(url: components.url!)
        request.timeoutInterval = 15
        request.setValue("frigo-vide-ios/0.1", forHTTPHeaderField: "User-Agent")
        let (data, response) = try await session.data(for: request)
        if let http = response as? HTTPURLResponse, !(200..<300).contains(http.statusCode) {
            throw Failure.http(http.statusCode)
        }
        return data
    }

    /// Les recettes qui contiennent cet ingrédient (en anglais ou en français).
    public func byIngredient(_ ingredient: String) async throws -> [MealSummary] {
        try parseSummaries(await get("filter.php", ["i": Names.apiName(ingredient)]))
    }

    /// Le détail d'une recette.
    public func lookup(_ id: String) async throws -> Meal? {
        try parseMeals(await get("lookup.php", ["i": id])).first
    }

    /// Une recette au hasard.
    public func random() async throws -> Meal? {
        try parseMeals(await get("random.php", [:])).first
    }

    /// Les recettes possibles avec ce frigo, de la meilleure à la moins bonne.
    public func suggest(fridge: [String], criteria: Criteria = Criteria()) async throws -> [Match] {
        guard !fridge.isEmpty else { throw Failure.emptyFridge }

        // 1. Chaque ingrédient en parallèle : combien d'ingrédients trouvent chaque recette.
        let lists = try await limited(fridge) { item in try await self.byIngredient(item) }
        var hits: [String: (count: Int, order: Int)] = [:]
        var order = 0
        for list in lists {
            for summary in list {
                order += 1
                hits[summary.id, default: (0, order)].count += 1
            }
        }
        let ids = hits
            .sorted { $0.value.count != $1.value.count ? $0.value.count > $1.value.count : $0.value.order < $1.value.order }
            .prefix(maxDetails)
            .map(\.key)

        // 2. Le détail des meilleures, en parallèle ; 3. le classement, en local.
        let meals = try await limited(ids) { id in try await self.lookup(id) }.compactMap { $0 }
        return rank(meals, fridge: fridge, criteria: criteria)
    }

    /// `work` sur chaque élément, au plus `concurrency` à la fois, résultats dans l'ordre.
    func limited<T: Sendable, R: Sendable>(
        _ items: [T],
        _ work: @escaping @Sendable (T) async throws -> R
    ) async throws -> [R] {
        let width = max(concurrency, 1)
        return try await withThrowingTaskGroup(of: (Int, R).self) { group in
            var results = [R?](repeating: nil, count: items.count)
            var next = 0
            for _ in 0..<min(width, items.count) {
                let (index, item) = (next, items[next])
                group.addTask { (index, try await work(item)) }
                next += 1
            }
            while let (index, result) = try await group.next() {
                results[index] = result
                if next < items.count {
                    let (index, item) = (next, items[next])
                    group.addTask { (index, try await work(item)) }
                    next += 1
                }
            }
            return results.compactMap { $0 }
        }
    }
}
