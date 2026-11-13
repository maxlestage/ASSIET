import FrigoKit
import SwiftData
import SwiftUI

/// L'écran principal : le frigo (étiquettes), les suggestions, et les recettes.
struct ContentView: View {
    @Environment(\.modelContext) private var context
    @Query(sort: \FridgeItem.addedAt) private var items: [FridgeItem]
    @State private var kitchen = Kitchen()
    @State private var draft = ""
    @FocusState private var typing: Bool
    @Namespace private var cards

    /// Des idées d'ingrédients, dans la langue de l'app : FrigoKit comprend le français,
    /// l'anglais et l'espagnol (voir `Localizable.xcstrings`).
    static let suggestions: [String] = [
        String(localized: "poulet", comment: "Suggestion d'ingrédient, en minuscules"),
        String(localized: "riz", comment: "Suggestion d'ingrédient, en minuscules"),
        String(localized: "oignon", comment: "Suggestion d'ingrédient, en minuscules"),
        String(localized: "ail", comment: "Suggestion d'ingrédient, en minuscules"),
        String(localized: "tomate", comment: "Suggestion d'ingrédient, en minuscules"),
        String(localized: "œufs", comment: "Suggestion d'ingrédient, en minuscules"),
        String(localized: "pâtes", comment: "Suggestion d'ingrédient, en minuscules"),
        String(localized: "fromage", comment: "Suggestion d'ingrédient, en minuscules"),
        String(localized: "carotte", comment: "Suggestion d'ingrédient, en minuscules"),
        String(localized: "pomme de terre", comment: "Suggestion d'ingrédient, en minuscules"),
        String(localized: "lait", comment: "Suggestion d'ingrédient, en minuscules"),
        String(localized: "citron", comment: "Suggestion d'ingrédient, en minuscules"),
    ]

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 24) {
                    FridgeHeader(count: items.count, searching: kitchen.phase == .searching)
                    fridge
                    results
                }
                .padding(.horizontal)
                .padding(.bottom, 40)
            }
            .scrollDismissesKeyboard(.interactively)
            .background(Backdrop())
            .navigationTitle("Mon frigo")
            .navigationDestination(for: Match.self) { match in
                RecipeDetailView(match: match)
                    .navigationTransition(.zoom(sourceID: match.id, in: cards))
            }
            .toolbar {
                if !items.isEmpty {
                    Button("Vider", systemImage: "trash", role: .destructive) {
                        withAnimation(.snappy) { items.forEach { context.delete($0) } }
                    }
                }
            }
        }
        .sensoryFeedback(.success, trigger: kitchen.matches.count) { _, new in new > 0 }
        .sensoryFeedback(.error, trigger: kitchen.phase) { _, new in
            if case .failed = new { true } else { false }
        }
    }

    // MARK: Le frigo

    private var fridge: some View {
        VStack(alignment: .leading, spacing: 16) {
            FlowLayout(spacing: 8) {
                ForEach(items) { item in
                    Button {
                        withAnimation(.spring(bounce: 0.4)) { context.delete(item) }
                    } label: {
                        Label(item.name, systemImage: "xmark")
                            .labelStyle(TrailingIconLabelStyle())
                    }
                    .buttonStyle(ChipStyle(tint: .mint))
                    .transition(.scale(scale: 0.3).combined(with: .opacity))
                    .accessibilityLabel(Text("Retirer \(item.name)"))
                }
                if items.isEmpty {
                    Text("Le frigo est vide…").foregroundStyle(.secondary)
                }
            }

            HStack {
                TextField("Ajouter un ingrédient…", text: $draft)
                    .textInputAutocapitalization(.never)
                    .autocorrectionDisabled()
                    .submitLabel(.done)
                    .focused($typing)
                    .onSubmit { add(draft) }
                    .padding(.horizontal, 16)
                    .frame(minHeight: 46)
                    .glassEffect(.regular, in: .capsule)
                Button("Ajouter") { add(draft) }
                    .buttonStyle(.glass)
                    .disabled(draft.trimmingCharacters(in: .whitespaces).isEmpty)
            }

            FlowLayout(spacing: 6) {
                ForEach(Self.suggestions, id: \.self) { suggestion in
                    let isIn = items.contains { $0.name == suggestion }
                    Button {
                        add(suggestion)
                    } label: {
                        Text(verbatim: "+ \(suggestion)")
                    }
                    .buttonStyle(ChipStyle(tint: .secondary))
                    .accessibilityLabel(Text("Ajouter \(suggestion)"))
                    .opacity(isIn ? 0.4 : 1)
                    .disabled(isIn)
                }
            }

            Toggle("Sans courses (strict)", isOn: $kitchen.strict)
            Toggle("Sel, huile, eau au placard", isOn: $kitchen.pantry)

            Button {
                typing = false
                kitchen.search(items.map(\.name))
            } label: {
                let title: LocalizedStringKey = kitchen.phase == .searching ? "Je fouille…" : "Trouver une recette"
                Label(title, systemImage: "refrigerator.fill")
                .symbolEffect(.bounce, value: kitchen.phase == .searching)
                .frame(maxWidth: .infinity, minHeight: 34)
            }
            .buttonStyle(.glassProminent)
            .tint(.orange)
            .disabled(kitchen.phase == .searching)
        }
        .padding(20)
        .glassEffect(.regular, in: .rect(cornerRadius: 28))
    }

    private func add(_ text: String) {
        let name = text.trimmingCharacters(in: .whitespacesAndNewlines).lowercased()
        draft = ""
        guard !name.isEmpty, !items.contains(where: { $0.name == name }) else { return }
        withAnimation(.spring(bounce: 0.5)) { context.insert(FridgeItem(name: name)) }
    }

    // MARK: Les recettes

    @ViewBuilder
    private var results: some View {
        switch kitchen.phase {
        case .idle:
            EmptyPlate()
        case .searching:
            ProgressView("TheMealDB fouille son garde-manger…")
                .frame(maxWidth: .infinity, minHeight: 160)
        case .failed(let message):
            Label(message, systemImage: "exclamationmark.triangle")
                .foregroundStyle(.orange)
        case .done where kitchen.matches.isEmpty:
            ContentUnavailableView(
                "Aucune recette",
                systemImage: "fork.knife",
                description: Text("Essayez sans le mode strict, ou ajoutez un ingrédient.")
            )
        case .done:
            LazyVStack(spacing: 18) {
                ForEach(Array(kitchen.matches.enumerated()), id: \.element.id) { index, match in
                    NavigationLink(value: match) {
                        RecipeCard(match: match, best: index == 0)
                            .matchedTransitionSource(id: match.id, in: cards)
                    }
                    .buttonStyle(.plain)
                    // La carte arrive en 3D quand elle entre dans l'écran.
                    .scrollTransition(.interactive, axis: .vertical) { content, phase in
                        content
                            .rotation3DEffect(.degrees(phase.value * -18), axis: (x: 1, y: 0, z: 0), perspective: 0.6)
                            .scaleEffect(1 - abs(phase.value) * 0.08)
                            .opacity(1 - abs(phase.value) * 0.4)
                    }
                }
            }
        }
    }
}

/// L'en-tête : un frigo en 3D qui tourne doucement, et s'ouvre pendant la recherche.
struct FridgeHeader: View {
    let count: Int
    let searching: Bool

    var body: some View {
        HStack(spacing: 16) {
            Image(systemName: searching ? "refrigerator.fill" : "refrigerator")
                .font(.system(size: 44))
                .foregroundStyle(.linearGradient(colors: [.orange, .pink], startPoint: .top, endPoint: .bottom))
                .contentTransition(.symbolEffect(.replace))
                .phaseAnimator([0.0, 1.0]) { content, phase in
                    content.rotation3DEffect(.degrees(-20 + phase * 40), axis: (x: 0, y: 1, z: 0), perspective: 0.5)
                } animation: { _ in .easeInOut(duration: 2.4) }
            VStack(alignment: .leading) {
                Text("Le frigo est vide ?").font(.title2.bold())
                Text(subtitle)
                    .foregroundStyle(.secondary)
                    .contentTransition(.numericText())
            }
        }
        .padding(.top, 8)
    }

    /// « 1 ingrédient » / « 2 ingrédients » : le pluriel est dans `Localizable.xcstrings`.
    private var subtitle: LocalizedStringKey {
        count == 0 ? "Pas l'assiette." : "\(count) ingrédients au frais"
    }
}

/// Une assiette vide qui tourne en 3D, en attendant les recettes.
struct EmptyPlate: View {
    var body: some View {
        VStack(spacing: 14) {
            ZStack {
                Circle().fill(.white.opacity(0.92))
                Circle().strokeBorder(.yellow.opacity(0.8), lineWidth: 4)
                Circle().fill(.white).padding(26)
            }
            .frame(width: 130, height: 130)
            .phaseAnimator([0.0, 1.0]) { content, phase in
                content
                    .rotation3DEffect(.degrees(58), axis: (x: 1, y: 0, z: 0))
                    .rotationEffect(.degrees(phase * 180))
                    .offset(y: phase * -6)
            } animation: { _ in .easeInOut(duration: 3) }
            Text("Les recettes apparaîtront ici, de la meilleure à la moins bonne.")
                .multilineTextAlignment(.center)
                .foregroundStyle(.secondary)
        }
        .frame(maxWidth: .infinity)
        .padding(.vertical, 24)
    }
}

/// Le fond : des taches de couleur qui dérivent lentement.
struct Backdrop: View {
    var body: some View {
        TimelineView(.animation(minimumInterval: 1 / 30)) { timeline in
            let t = timeline.date.timeIntervalSinceReferenceDate
            MeshGradient(
                width: 3,
                height: 3,
                points: [
                    [0, 0], [0.5, 0], [1, 0],
                    [0, 0.5], [Float(0.5 + 0.15 * sin(t / 3)), Float(0.5 + 0.1 * cos(t / 4))], [1, 0.5],
                    [0, 1], [0.5, 1], [1, 1],
                ],
                colors: [
                    .black, .indigo.opacity(0.5), .black,
                    .pink.opacity(0.35), .black, .purple.opacity(0.35),
                    .black, .teal.opacity(0.3), .black,
                ]
            )
        }
        .ignoresSafeArea()
    }
}
