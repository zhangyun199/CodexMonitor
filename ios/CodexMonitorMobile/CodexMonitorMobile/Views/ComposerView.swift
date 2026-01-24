import SwiftUI
import PhotosUI
import UniformTypeIdentifiers
import CodexMonitorModels

struct ComposerView: View {
    @EnvironmentObject private var store: CodexStore
    let workspaceId: String
    let threadId: String

    @State private var message = ""
    @State private var accessMode: AccessMode = .current
    @State private var attachedImages: [Data] = []
    @State private var photoSelection: [PhotosPickerItem] = []
    @State private var showFileImporter = false
    @StateObject private var dictation = DictationController()

    var body: some View {
        VStack(spacing: 8) {
            HStack {
                TextEditor(text: $message)
                    .frame(minHeight: 60, maxHeight: 120)
                    .overlay(RoundedRectangle(cornerRadius: 8).stroke(Color.secondary.opacity(0.2)))
                Button(action: send) {
                    Image(systemName: "paperplane.fill")
                        .padding(10)
                }
                .buttonStyle(.borderedProminent)
            }

            ScrollView(.horizontal, showsIndicators: false) {
                HStack(spacing: 8) {
                    PhotosPicker(selection: $photoSelection, matching: .images, photoLibrary: .shared()) {
                        Label("Photos", systemImage: "photo")
                    }
                    .buttonStyle(.bordered)

                    Button(action: { showFileImporter = true }) {
                        Label("Files", systemImage: "paperclip")
                    }
                    .buttonStyle(.bordered)

                    Button(action: pasteImage) {
                        Label("Paste", systemImage: "doc.on.clipboard")
                    }
                    .buttonStyle(.bordered)

                    Button(action: { dictation.toggle() }) {
                        Label(dictation.isRecording ? "Stop" : "Dictate", systemImage: dictation.isRecording ? "mic.fill" : "mic")
                    }
                    .buttonStyle(.bordered)

                    Picker("Access", selection: $accessMode) {
                        Text("Read").tag(AccessMode.readOnly)
                        Text("Current").tag(AccessMode.current)
                        Text("Full").tag(AccessMode.fullAccess)
                    }
                    .pickerStyle(.segmented)
                }
            }
            .frame(maxHeight: 32)

            if !attachedImages.isEmpty {
                ScrollView(.horizontal, showsIndicators: false) {
                    HStack {
                        ForEach(Array(attachedImages.enumerated()), id: \.offset) { index, data in
                            if let uiImage = UIImage(data: data) {
                                Image(uiImage: uiImage)
                                    .resizable()
                                    .scaledToFill()
                                    .frame(width: 48, height: 48)
                                    .clipShape(RoundedRectangle(cornerRadius: 8))
                                    .overlay(alignment: .topTrailing) {
                                        Button(action: { attachedImages.remove(at: index) }) {
                                            Image(systemName: "xmark.circle.fill")
                                        }
                                    }
                            }
                        }
                    }
                }
            }

            if dictation.isRecording || !dictation.transcript.isEmpty {
                GlassCard {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Dictation")
                            .font(.caption.weight(.semibold))
                        Text(dictation.transcript.isEmpty ? "Listening..." : dictation.transcript)
                            .font(.caption)
                    }
                }
            }
        }
        .fileImporter(isPresented: $showFileImporter, allowedContentTypes: [.image]) { result in
            if case .success(let url) = result, let data = try? Data(contentsOf: url) {
                attachedImages.append(data)
            }
        }
        .onChange(of: photoSelection) { _, newSelection in
            Task {
                for item in newSelection {
                    if let data = try? await item.loadTransferable(type: Data.self) {
                        attachedImages.append(data)
                    }
                }
                photoSelection.removeAll()
            }
        }
        .onChange(of: dictation.isRecording) { _, isRecording in
            if !isRecording, !dictation.transcript.isEmpty {
                message += (message.isEmpty ? "" : "\n") + dictation.transcript
                dictation.transcript = ""
            }
        }
    }

    private func send() {
        let text = message.trimmingCharacters(in: .whitespacesAndNewlines)
        let images = attachedImages.map { data -> String in
            let base64 = data.base64EncodedString()
            return "data:image/jpeg;base64,\(base64)"
        }
        guard !text.isEmpty || !images.isEmpty else { return }
        message = ""
        attachedImages.removeAll()
        Task {
            await store.sendMessage(
                workspaceId: workspaceId,
                threadId: threadId,
                text: text,
                accessMode: accessMode,
                images: images
            )
        }
    }

    private func pasteImage() {
        if let image = UIPasteboard.general.image,
           let data = image.jpegData(compressionQuality: 0.85) {
            attachedImages.append(data)
        }
    }
}
