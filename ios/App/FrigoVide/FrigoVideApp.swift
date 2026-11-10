import SwiftData
import SwiftUI

/// Frigo vide sur iPhone : le frigo est gardé sur l'appareil (SwiftData), les recettes
/// viennent de TheMealDB via FrigoKit.
@main
struct FrigoVideApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
        .modelContainer(for: FridgeItem.self)
    }
}

/// Un ingrédient du frigo, enregistré sur l'appareil.
@Model
final class FridgeItem {
    @Attribute(.unique) var name: String
    var addedAt: Date

    init(name: String, addedAt: Date = .now) {
        self.name = name
        self.addedAt = addedAt
    }
}
