import FrigoKit
import SwiftUI

/// Une recette dans la liste : la photo, le temps, ce qui manque, la jauge du frigo.
struct RecipeCard: View {
    let match: Match
    let best: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            AsyncImage(url: match.meal.thumbnail.map { $0.appendingPathComponent("medium") }) { image in
                image.resizable().scaledToFill()
            } placeholder: {
                LinearGradient(colors: [.orange.opacity(0.5), .pink.opacity(0.4)], startPoint: .topLeading, endPoint: .bottomTrailing)
            }
            .frame(height: 180)
            .clipped()
            .overlay(alignment: .topLeading) {
                if best {
                    Text("Meilleur choix")
                        .font(.caption.bold())
                        .padding(.horizontal, 10)
                        .padding(.vertical, 5)
                        .background(.yellow.gradient, in: .capsule)
                        .foregroundStyle(.black)
                        .padding(12)
                }
            }

            VStack(alignment: .leading, spacing: 8) {
                HStack(spacing: 6) {
                    Tag(text: "≈ \(match.minutes) min", systemImage: "timer", tint: .purple)
                    if match.isComplete {
                        Tag(text: "Rien à acheter", systemImage: "checkmark", tint: .mint)
                    } else {
                        Tag(text: "Manque \(match.missing.count)", systemImage: "cart", tint: .orange)
                    }
                }
                Text(match.meal.name).font(.headline)
                Text([match.meal.category, match.meal.area].compactMap(\.self).joined(separator: " · "))
                    .font(.subheadline)
                    .foregroundStyle(.secondary)
                Gauge(value: match.coverage) {
                    Text("Frigo")
                }
                .gaugeStyle(.accessoryLinearCapacity)
                .tint(.linearGradient(colors: [.orange, .yellow, .mint], startPoint: .leading, endPoint: .trailing))
            }
            .padding(16)
        }
        .glassEffect(.regular, in: .rect(cornerRadius: 24))
        .clipShape(.rect(cornerRadius: 24))
        .overlay {
            if best {
                RoundedRectangle(cornerRadius: 24).strokeBorder(.yellow.opacity(0.6), lineWidth: 1.5)
            }
        }
        .accessibilityElement(children: .combine)
    }
}

struct Tag: View {
    let text: String
    let systemImage: String
    let tint: Color

    var body: some View {
        Label(text, systemImage: systemImage)
            .font(.caption.weight(.semibold))
            .padding(.horizontal, 9)
            .padding(.vertical, 4)
            .foregroundStyle(tint)
            .background(tint.opacity(0.14), in: .capsule)
    }
}

/// Le détail d'une recette : ce que j'ai, ce qui manque, les étapes.
struct RecipeDetailView: View {
    let match: Match
    @State private var done: Set<Int> = []

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 22) {
                AsyncImage(url: match.meal.thumbnail) { image in
                    image.resizable().scaledToFill()
                } placeholder: {
                    Color.orange.opacity(0.3)
                }
                .frame(height: 280)
                .clipShape(.rect(cornerRadius: 28))
                .visualEffect { content, proxy in
                    // Un léger effet de profondeur quand on fait défiler.
                    let y = proxy.frame(in: .scrollView).minY
                    return content
                        .rotation3DEffect(.degrees(min(max(y / 12, -10), 10)), axis: (x: 1, y: 0, z: 0))
                        .scaleEffect(1 + max(y, 0) / 900)
                }

                HStack {
                    Tag(text: "≈ \(match.minutes) min", systemImage: "timer", tint: .purple)
                    Tag(text: "\(Int((match.coverage * 100).rounded())) % du frigo", systemImage: "refrigerator", tint: .mint)
                }

                section("Il vous faut") {
                    ForEach(match.meal.ingredients, id: \.self) { ingredient in
                        let ok = match.have.contains(ingredient.name)
                        HStack {
                            Image(systemName: ok ? "checkmark.circle.fill" : "xmark.circle")
                                .foregroundStyle(ok ? .mint : .orange)
                            Text(ingredient.name)
                            Spacer()
                            Text(ingredient.measure).foregroundStyle(.secondary)
                        }
                    }
                }

                section("Étapes") {
                    ForEach(Array(match.meal.steps.enumerated()), id: \.offset) { index, step in
                        Button {
                            withAnimation(.snappy) {
                                if done.contains(index) { done.remove(index) } else { done.insert(index) }
                            }
                        } label: {
                            HStack(alignment: .top, spacing: 12) {
                                Text("\(index + 1)")
                                    .font(.headline.monospacedDigit())
                                    .frame(width: 30, height: 30)
                                    .background(done.contains(index) ? .mint : .orange.opacity(0.25), in: .circle)
                                Text(step)
                                    .strikethrough(done.contains(index))
                                    .foregroundStyle(done.contains(index) ? .secondary : .primary)
                                    .multilineTextAlignment(.leading)
                            }
                        }
                        .buttonStyle(.plain)
                        .sensoryFeedback(.selection, trigger: done.contains(index))
                    }
                }

                if let video = match.meal.youtube {
                    Link(destination: video) {
                        Label("Voir la vidéo", systemImage: "play.rectangle.fill")
                            .frame(maxWidth: .infinity, minHeight: 34)
                    }
                    .buttonStyle(.glassProminent)
                    .tint(.red)
                }
            }
            .padding()
        }
        .background(Backdrop())
        .navigationTitle(match.meal.name)
        .navigationBarTitleDisplayMode(.inline)
    }

    private func section(_ title: String, @ViewBuilder content: () -> some View) -> some View {
        VStack(alignment: .leading, spacing: 10) {
            Text(title).font(.title3.bold())
            content()
        }
        .padding(18)
        .frame(maxWidth: .infinity, alignment: .leading)
        .glassEffect(.regular, in: .rect(cornerRadius: 24))
    }
}
