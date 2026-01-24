import XCTest
@testable import CodexMonitorModels

final class JSONValueTests: XCTestCase {
    func testJSONValueRoundTrip() throws {
        let value: JSONValue = .object([
            "name": .string("Codex"),
            "count": .number(3),
            "flags": .array([.bool(true), .null]),
        ])
        let data = try JSONEncoder().encode(value)
        let decoded = try JSONDecoder().decode(JSONValue.self, from: data)
        XCTAssertEqual(decoded, value)
    }

    func testMergeStreamingText() {
        let base = "Hello wor"
        let delta = "world"
        let merged = ConversationHelpers.mergeStreamingText(existing: base, delta: delta)
        XCTAssertEqual(merged, "Hello world")
    }
}
