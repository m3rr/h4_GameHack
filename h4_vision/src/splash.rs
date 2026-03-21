use slint::ComponentHandle;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub fn start_splash_driver<T: ComponentHandle + 'static>(ui_handle: slint::Weak<T>) {
    tokio::spawn(async move {
        let set_splash = move |u: &T, opacity: f32, alpha: f32, j_x: f32, j_y: f32, rgb: f32| {
            u.set_property("logo_opacity", opacity.into()).unwrap_or_default();
            u.set_property("glitch_alpha", alpha.into()).unwrap_or_default();
            u.set_property("jitter_x", slint::LogicalLength::new(j_x).into()).unwrap_or_default();
            u.set_property("jitter_y", slint::LogicalLength::new(j_y).into()).unwrap_or_default();
            u.set_property("r_offset", slint::LogicalLength::new(rgb).into()).unwrap_or_default();
            u.set_property("b_offset", slint::LogicalLength::new(-rgb).into()).unwrap_or_default();
        };

        let ui_set = ui_handle.clone();
        
        // Use set_splash via invoke_from_event_loop
        let update = move |opacity: f32, alpha: f32, j_x: f32, j_y: f32, rgb: f32| {
            let u_weak = ui_set.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(u) = u_weak.upgrade() {
                    // We must use specific property setters if we want type safety, 
                    // or set_property if we want generic. 
                    // Since MainWindow is a generated type, we prefer the specific setters.
                    // But to stay generic for any component handle, we use set_property.
                    // Actually, for this project, MainWindow is the only user.
                }
            });
        };
        // wait, the generic approach is hard with slint because properties are generated.
        // I will keep it in main.rs but just organize it better or use a macro.
    });
}
