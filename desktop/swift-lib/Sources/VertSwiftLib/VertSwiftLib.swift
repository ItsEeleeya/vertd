import AppKit
import SwiftUI

struct VisualEffectView: NSViewRepresentable {
    let material: NSVisualEffectView.Material
    let blendingMode: NSVisualEffectView.BlendingMode

    func makeNSView(context: Context) -> NSVisualEffectView {
        let view = NSVisualEffectView()
        view.material = material
        view.blendingMode = blendingMode
        view.state = .active
        return view
    }

    func updateNSView(_ nsView: NSVisualEffectView, context: Context) {
        nsView.material = material
        nsView.blendingMode = blendingMode
    }
}

@_cdecl("apply_swiftui_window_chrome")
@MainActor
func applySwiftUIWindowChrome(to nswindowPtr: UnsafeRawPointer, radius cornerRadius: Double) {
    let window: NSWindow = Unmanaged<NSWindow>.fromOpaque(nswindowPtr).takeUnretainedValue()

    window.isOpaque = false
    window.backgroundColor = .clear
    window.titlebarAppearsTransparent = true
    window.titleVisibility = .hidden
    window.hasShadow = true

    guard let contentView = window.contentView else {
        print("Error: Window has no contentView")
        return
    }
    contentView.wantsLayer = true
    contentView.layer?.cornerRadius = cornerRadius
    contentView.layer?.masksToBounds = true

    var body: some View {
        VisualEffectView(material: .underWindowBackground, blendingMode: .behindWindow)
            .clipShape(RoundedRectangle(cornerRadius: cornerRadius))
            .overlay(
                RoundedRectangle(cornerRadius: cornerRadius)
                    .strokeBorder(Color.white.opacity(0.25), lineWidth: 1.0)
            )
            .ignoresSafeArea()
    }

    // Host a NSView containing the SwiftUI View in AppKit
    let chromeView = NSHostingView(rootView: body)
    chromeView.translatesAutoresizingMaskIntoConstraints = false
    chromeView.layer?.backgroundColor = NSColor.clear.cgColor

    contentView.addSubview(chromeView, positioned: .below, relativeTo: nil)

    NSLayoutConstraint.activate([
        chromeView.topAnchor.constraint(equalTo: contentView.topAnchor),
        chromeView.bottomAnchor.constraint(equalTo: contentView.bottomAnchor),
        chromeView.leadingAnchor.constraint(equalTo: contentView.leadingAnchor),
        chromeView.trailingAnchor.constraint(equalTo: contentView.trailingAnchor),
    ])
}
