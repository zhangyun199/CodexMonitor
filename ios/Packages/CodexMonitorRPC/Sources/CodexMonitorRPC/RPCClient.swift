import Foundation
import Network
import CodexMonitorModels

public actor RPCClient {
    public struct Config: Sendable {
        public var host: String
        public var port: UInt16
        public var token: String

        public init(host: String, port: UInt16, token: String) {
            self.host = host
            self.port = port
            self.token = token
        }
    }

    public enum RPCClientError: Error, Sendable {
        case disconnected
        case invalidPort
        case invalidResponse
        case remoteError(String)
    }

    private var connection: NWConnection?
    private var nextID: UInt64 = 2
    private var pending: [UInt64: CheckedContinuation<JSONValue, Error>] = [:]
    private var buffer = Data()
    private var receiveTask: Task<Void, Never>?

    public var onNotification: (@Sendable (RPCNotification) -> Void)?

    public init() {}

    public func connect(_ config: Config) async throws {
        disconnect()

        guard let port = NWEndpoint.Port(rawValue: config.port) else {
            throw RPCClientError.invalidPort
        }

        let host = NWEndpoint.Host(config.host)
        let connection = NWConnection(host: host, port: port, using: .tcp)
        self.connection = connection

        try await withCheckedThrowingContinuation { continuation in
            connection.stateUpdateHandler = { state in
                switch state {
                case .ready:
                    connection.stateUpdateHandler = nil
                    continuation.resume()
                case .failed(let error):
                    connection.stateUpdateHandler = nil
                    continuation.resume(throwing: error)
                case .cancelled:
                    connection.stateUpdateHandler = nil
                    continuation.resume(throwing: RPCClientError.disconnected)
                default:
                    break
                }
            }
            connection.start(queue: .global())
        }

        startReceiveLoop()
        nextID = 2
        let authParams = JSONValue.object(["token": .string(config.token)])
        _ = try await callWithID(1, method: "auth", params: authParams)
    }

    public func disconnect() {
        receiveTask?.cancel()
        receiveTask = nil
        connection?.cancel()
        connection = nil
        failPending(with: RPCClientError.disconnected)
    }

    public func call(method: String, params: JSONValue? = nil) async throws -> JSONValue {
        let id = nextID
        nextID += 1
        return try await callWithID(id, method: method, params: params)
    }

    private func callWithID(_ id: UInt64, method: String, params: JSONValue?) async throws -> JSONValue {
        guard connection != nil else {
            throw RPCClientError.disconnected
        }
        let request = RPCRequest(id: id, method: method, params: params)
        return try await withCheckedThrowingContinuation { continuation in
            pending[id] = continuation
            sendLine(request)
        }
    }

    private func sendLine<T: Encodable>(_ value: T) {
        guard let connection else { return }
        do {
            let data = try JSONEncoder().encode(value)
            var line = data
            line.append(0x0A)
            connection.send(content: line, completion: .contentProcessed { [weak self] error in
                guard let error else { return }
                Task { await self?.handleSendError(error) }
            })
        } catch {
            failPending(with: error)
        }
    }

    private func startReceiveLoop() {
        receiveTask = Task { [weak self] in
            await self?.receiveNext()
        }
    }

    private func receiveNext() async {
        guard let connection else { return }
        connection.receive(minimumIncompleteLength: 1, maximumLength: 64 * 1024) { [weak self] data, _, isComplete, error in
            Task { await self?.handleReceive(data: data, isComplete: isComplete, error: error) }
        }
    }

    private func handleReceive(data: Data?, isComplete: Bool, error: NWError?) async {
        if let error {
            failPending(with: error)
            return
        }

        if let data, !data.isEmpty {
            buffer.append(data)
            processBuffer()
        }

        if isComplete {
            failPending(with: RPCClientError.disconnected)
            return
        }

        await receiveNext()
    }

    private func processBuffer() {
        while let range = buffer.firstRange(of: Data([0x0A])) {
            let lineData = buffer.subdata(in: 0..<range.lowerBound)
            buffer.removeSubrange(0...range.lowerBound)
            guard !lineData.isEmpty else { continue }
            decodeLine(lineData)
        }
    }

    private func decodeLine(_ data: Data) {
        guard let envelope = try? JSONDecoder().decode(RPCEnvelope.self, from: data) else {
            return
        }

        if let id = envelope.id {
            if let continuation = pending.removeValue(forKey: id) {
                if let error = envelope.error {
                    continuation.resume(throwing: RPCClientError.remoteError(error.message))
                } else {
                    continuation.resume(returning: envelope.result ?? .null)
                }
            }
            return
        }

        if let method = envelope.method {
            let notification = RPCNotification(method: method, params: envelope.params)
            onNotification?(notification)
        }
    }

    private func handleSendError(_ error: Error) {
        failPending(with: error)
    }

    private func failPending(with error: Error) {
        let pending = self.pending
        self.pending.removeAll()
        for (_, continuation) in pending {
            continuation.resume(throwing: error)
        }
    }
}
