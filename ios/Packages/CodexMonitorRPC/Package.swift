// swift-tools-version: 5.10
import PackageDescription

let package = Package(
    name: "CodexMonitorRPC",
    platforms: [
        .iOS(.v17),
        .macOS(.v12),
    ],
    products: [
        .library(name: "CodexMonitorRPC", targets: ["CodexMonitorRPC"]),
        .library(name: "CodexMonitorModels", targets: ["CodexMonitorModels"]),
        .library(name: "CodexMonitorRendering", targets: ["CodexMonitorRendering"]),
    ],
    targets: [
        .target(
            name: "CodexMonitorModels",
            path: "Sources/CodexMonitorModels"
        ),
        .target(
            name: "CodexMonitorRendering",
            dependencies: ["CodexMonitorModels"],
            path: "Sources/CodexMonitorRendering"
        ),
        .target(
            name: "CodexMonitorRPC",
            dependencies: ["CodexMonitorModels"],
            path: "Sources/CodexMonitorRPC"
        ),
        .testTarget(
            name: "CodexMonitorRPCTests",
            dependencies: ["CodexMonitorRPC", "CodexMonitorModels"],
            path: "Tests/CodexMonitorRPCTests"
        ),
    ]
)
