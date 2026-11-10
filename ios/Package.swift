// swift-tools-version: 6.4
// FrigoKit : le cœur de Frigo vide en Swift (modèles TheMealDB, normalisation des
// ingrédients, classement, client asynchrone). Sans dépendance, testé avec Swift Testing,
// compilable sur iOS, macOS et Linux. L'app SwiftUI (App/) l'utilise via project.yml.
import PackageDescription

let package = Package(
    name: "FrigoKit",
    defaultLocalization: "fr",
    platforms: [.iOS(.v26), .macOS(.v26)],
    products: [
        .library(name: "FrigoKit", targets: ["FrigoKit"]),
    ],
    targets: [
        .target(name: "FrigoKit"),
        .testTarget(
            name: "FrigoKitTests",
            dependencies: ["FrigoKit"],
            resources: [.copy("Fixtures")]
        ),
    ],
    swiftLanguageModes: [.v6]
)
