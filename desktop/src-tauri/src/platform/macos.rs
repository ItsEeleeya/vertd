use objc2::{MainThreadMarker, msg_send};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSColor, NSView, NSVisualEffectBlendingMode, NSVisualEffectMaterial,
    NSVisualEffectState, NSVisualEffectView, NSWindow, NSWindowOrderingMode,
};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};

use crate::{AppError, AppResult};

use swift_rs::swift;

swift!(pub fn say_hello());

pub unsafe fn set_window_effects(
    ns_window: &NSWindow,
    material: NSVisualEffectMaterial,
    state: Option<NSVisualEffectState>,
    radius: Option<f64>,
) -> AppResult<()> {
    let mtm = MainThreadMarker::new().ok_or(AppError::Anyhow(anyhow::anyhow!(
        "\"set_window_radius()\" can only be used on the main thread."
    )))?;

    unsafe {
        ns_window.setBackgroundColor(Some(&NSColor::clearColor()));

        let content_view = ns_window
            .contentView()
            .ok_or(AppError::Anyhow(anyhow::anyhow!(
                "No content view for window"
            )))?;

        let effect = NSVisualEffectView::initWithFrame(mtm.alloc(), content_view.bounds());
        effect.setMaterial(material);
        effect.setState(state.unwrap_or(NSVisualEffectState::Active));
        effect.setBlendingMode(NSVisualEffectBlendingMode::BehindWindow);
        effect.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );
        effect.setWantsLayer(true);
        effect.setTranslatesAutoresizingMaskIntoConstraints(false);

        if let (Some(radius), Some(layer)) = (radius, effect.layer()) {
            println!("Setting radius {radius}");
            layer.setMasksToBounds(true);
            let _: () = msg_send![&layer, setCornerRadius: radius];

            println!("Corner curve: {:?}", layer.cornerCurve());
        }

        content_view.addSubview_positioned_relativeTo(&effect, NSWindowOrderingMode::Below, None);

        effect
            .leadingAnchor()
            .constraintEqualToAnchor(&content_view.leadingAnchor())
            .setActive(true);

        effect
            .trailingAnchor()
            .constraintEqualToAnchor(&content_view.trailingAnchor())
            .setActive(true);

        effect
            .topAnchor()
            .constraintEqualToAnchor(&content_view.topAnchor())
            .setActive(true);

        effect
            .bottomAnchor()
            .constraintEqualToAnchor(&content_view.bottomAnchor())
            .setActive(true);

        let border_view = NSView::initWithFrame(mtm.alloc(), content_view.bounds());
        // let border_rect = CGRect::new(CGPoint::new(0.0, 0.0), CGSize::new(200.0, 200.0));
        // border_view.drawRect(dirty_rect);

        content_view.addSubview_positioned_relativeTo(
            &border_view,
            NSWindowOrderingMode::Above,
            None,
        );
    }

    Ok(())
}
