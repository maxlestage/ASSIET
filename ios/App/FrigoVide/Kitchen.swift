import FrigoKit
import Observation
import SwiftUI

/// L'état de la recherche : ce que l'écran affiche, mis à jour sur le fil principal.
@MainActor
@Observable
final class Kitchen {
    enum Phase: Equatable {
        case idle
        case searching
        case done
        case failed(String)
    }

    private(set) var phase: Phase = .idle
    private(set) var matches: [Match] = []
    var strict = false
    var pantry = true

    private let db = MealDB()
    private var task: Task<Void, Never>?

    /// Cherche les recettes de ce frigo ; une recherche en cours est annulée.
    func search(_ fridge: [String]) {
        task?.cancel()
        guard !fridge.isEmpty else {
            phase = .failed(Self.message(for: MealDB.Failure.emptyFridge))
            return
        }
        phase = .searching
        let criteria = Criteria(strict: strict, pantry: pantry)
        task = Task {
            do {
                let found = try await db.suggest(fridge: fridge, criteria: criteria)
                guard !Task.isCancelled else { return }
                withAnimation(.spring(duration: 0.6, bounce: 0.3)) {
                    matches = Array(found.prefix(12))
                    phase = .done
                }
            } catch is CancellationError {
            } catch {
                phase = .failed(Self.message(for: error))
            }
        }
    }

    /// Le message d'erreur dans la langue de l'app : ceux de FrigoKit sont en français, ceux
    /// du système (réseau…) sont déjà traduits.
    private static func message(for error: any Error) -> String {
        switch error as? MealDB.Failure {
        case .emptyFridge?:
            String(localized: "Le frigo est vide : ajoutez au moins un ingrédient.")
        case .http(let status)?:
            String(localized: "TheMealDB a répondu \(status).")
        case nil:
            error.localizedDescription
        }
    }
}
