import Foundation
import CodexMonitorModels

public struct RPCRequest: Codable, Sendable {
    public let id: UInt64
    public let method: String
    public let params: JSONValue?

    public init(id: UInt64, method: String, params: JSONValue?) {
        self.id = id
        self.method = method
        self.params = params
    }
}

public struct RPCResponse: Codable, Sendable {
    public let id: UInt64
    public let result: JSONValue?
    public let error: RPCError?

    public init(id: UInt64, result: JSONValue?, error: RPCError?) {
        self.id = id
        self.result = result
        self.error = error
    }
}

public struct RPCNotification: Codable, Sendable {
    public let method: String
    public let params: JSONValue?

    public init(method: String, params: JSONValue?) {
        self.method = method
        self.params = params
    }
}

public struct RPCError: Codable, Error, Sendable {
    public let message: String

    public init(message: String) {
        self.message = message
    }
}

struct RPCEnvelope: Codable {
    let id: UInt64?
    let method: String?
    let params: JSONValue?
    let result: JSONValue?
    let error: RPCError?
}
