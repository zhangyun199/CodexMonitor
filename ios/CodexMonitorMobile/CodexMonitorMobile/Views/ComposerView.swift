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
    @State private var showImageError = false
    @State private var imageErrorMessage = ""

    private let maxImageBytes = 2 * 1024 * 1024
    private let maxImageDimension: CGFloat = 1920
    private let imageCompressionQuality: CGFloat = 0.7
    private let fallbackCompressionQuality: CGFloat = 0.4

    var body: some View {
        GlassPanel(cornerRadius: 20) {
            VStack(spacing: 8) {
                HStack(spacing: 12) {
                    TextEditor(text: $message)
                        .frame(minHeight: 60, maxHeight: 120)
                        .scrollContentBackground(.hidden)
                        .background(Color.clear)
                        .overlay(
                            RoundedRectangle(cornerRadius: 12)
                                .strokeBorder(Color.white.opacity(0.15), lineWidth: 1)
                        )

                    GlassSendButton(action: send)
                }

                GlassActionBar(
                    photoSelection: $photoSelection,
                    showFileImporter: $showFileImporter,
                    accessMode: $accessMode,
                    dictation: dictation,
                    pasteImage: pasteImage
                )

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
            .padding(14)
        }
        .fileImporter(isPresented: $showFileImporter, allowedContentTypes: [.image]) { result in
            if case .success(let url) = result, let data = try? Data(contentsOf: url) {
                handleIncomingImageData(data)
            }
        }
        .onChange(of: photoSelection) { _, newSelection in
            Task {
                for item in newSelection {
                    if let data = try? await item.loadTransferable(type: Data.self) {
                        handleIncomingImageData(data)
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
        .alert("Image Error", isPresented: $showImageError) {
            Button("OK", role: .cancel) {}
        } message: {
            Text(imageErrorMessage)
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
        if let image = UIPasteboard.general.image {
            handleIncomingImage(image)
        }
    }

    private func handleIncomingImageData(_ data: Data) {
        guard let image = UIImage(data: data) else {
            showImageError("Unsupported image format.")
            return
        }
        handleIncomingImage(image)
    }

    private func handleIncomingImage(_ image: UIImage) {
        guard let processed = processImageForUpload(image) else { return }
        attachedImages.append(processed)
    }

    private func processImageForUpload(_ image: UIImage) -> Data? {
        var processedImage = image
        let size = image.size
        if size.width > maxImageDimension || size.height > maxImageDimension {
            let scale = maxImageDimension / max(size.width, size.height)
            let newSize = CGSize(width: size.width * scale, height: size.height * scale)
            let renderer = UIGraphicsImageRenderer(size: newSize)
            processedImage = renderer.image { _ in
                image.draw(in: CGRect(origin: .zero, size: newSize))
            }
        }

        guard let data = processedImage.jpegData(compressionQuality: imageCompressionQuality) else {
            showImageError("Failed to process image.")
            return nil
        }

        if data.count > maxImageBytes {
            if let smallerData = processedImage.jpegData(compressionQuality: fallbackCompressionQuality),
               smallerData.count <= maxImageBytes {
                return smallerData
            }
            showImageError("Image too large. Max size is 2MB after compression.")
            return nil
        }

        return data
    }

    private func showImageError(_ message: String) {
        imageErrorMessage = message
        showImageError = true
    }
}

// MARK: - Glass Send Button
private struct GlassSendButton: View {
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            if #available(iOS 26.0, *) {
                Image(systemName: "paperplane.fill")
                    .font(.system(size: 18, weight: .semibold))
                    .foregroundStyle(.white)
                    .frame(width: 44, height: 44)
                    .glassEffect(.regular.tint(.accentColor).interactive(), in: .circle)
            } else {
                Image(systemName: "paperplane.fill")
                    .font(.system(size: 18, weight: .semibold))
                    .foregroundStyle(.white)
                    .frame(width: 44, height: 44)
                    .background(Color.accentColor, in: Circle())
            }
        }
        .buttonStyle(.plain)
    }
}

// MARK: - Glass Action Bar
private struct GlassActionBar: View {
    @Binding var photoSelection: [PhotosPickerItem]
    @Binding var showFileImporter: Bool
    @Binding var accessMode: AccessMode
    @ObservedObject var dictation: DictationController
    let pasteImage: () -> Void

    var body: some View {
        ScrollView(.horizontal, showsIndicators: false) {
            HStack(spacing: 8) {
                PhotosPicker(selection: $photoSelection, matching: .images, photoLibrary: .shared()) {
                    GlassActionButton(icon: "photo", label: "Photos")
                }

                Button(action: { showFileImporter = true }) {
                    GlassActionButton(icon: "paperclip", label: "Files")
                }

                Button(action: pasteImage) {
                    GlassActionButton(icon: "doc.on.clipboard", label: "Paste")
                }

                Button(action: { dictation.toggle() }) {
                    GlassActionButton(
                        icon: dictation.isRecording ? "mic.fill" : "mic",
                        label: dictation.isRecording ? "Stop" : "Dictate",
                        isActive: dictation.isRecording
                    )
                }

                Spacer()

                GlassAccessModePicker(selection: $accessMode)
            }
        }
        .frame(maxHeight: 36)
    }
}

private struct GlassActionButton: View {
    let icon: String
    let label: String
    var isActive: Bool = false

    var body: some View {
        if #available(iOS 26.0, *) {
            HStack(spacing: 4) {
                Image(systemName: icon)
                    .font(.system(size: 12, weight: .medium))
                Text(label)
                    .font(.caption.weight(.medium))
            }
            .foregroundStyle(isActive ? .red : .primary)
            .padding(.horizontal, 10)
            .padding(.vertical, 6)
            .glassEffect(isActive ? .regular.tint(.red).interactive() : .regular.interactive(), in: .capsule)
        } else {
            HStack(spacing: 4) {
                Image(systemName: icon)
                    .font(.system(size: 12, weight: .medium))
                Text(label)
                    .font(.caption.weight(.medium))
            }
            .foregroundStyle(isActive ? .red : .primary)
            .padding(.horizontal, 10)
            .padding(.vertical, 6)
            .background(.ultraThinMaterial, in: Capsule())
        }
    }
}

private struct GlassAccessModePicker: View {
    @Binding var selection: AccessMode

    var body: some View {
        if #available(iOS 26.0, *) {
            HStack(spacing: 2) {
                ForEach([AccessMode.readOnly, .current, .fullAccess], id: \.self) { mode in
                    Button(action: { selection = mode }) {
                        Text(labelFor(mode))
                            .font(.caption2.weight(.medium))
                            .padding(.horizontal, 8)
                            .padding(.vertical, 6)
                            .background(selection == mode ? Color.white.opacity(0.2) : Color.clear)
                            .clipShape(Capsule())
                    }
                    .buttonStyle(.plain)
                    .foregroundStyle(selection == mode ? .primary : .secondary)
                }
            }
            .glassEffect(.regular, in: .capsule)
        } else {
            HStack(spacing: 2) {
                ForEach([AccessMode.readOnly, .current, .fullAccess], id: \.self) { mode in
                    Button(action: { selection = mode }) {
                        Text(labelFor(mode))
                            .font(.caption2.weight(.medium))
                            .padding(.horizontal, 8)
                            .padding(.vertical, 6)
                            .background(selection == mode ? Color.white.opacity(0.2) : Color.clear)
                            .clipShape(Capsule())
                    }
                    .buttonStyle(.plain)
                    .foregroundStyle(selection == mode ? .primary : .secondary)
                }
            }
            .background(.ultraThinMaterial, in: Capsule())
        }
    }

    private func labelFor(_ mode: AccessMode) -> String {
        switch mode {
        case .readOnly: return "Read"
        case .current: return "Current"
        case .fullAccess: return "Full"
        }
    }
}
