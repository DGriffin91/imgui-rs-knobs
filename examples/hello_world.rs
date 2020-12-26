use imgui::*;
use imgui_knobs::*;
mod support;

fn main() {
    let system = support::init(file!());
    let mut value = 0.0;
    let min = -6.0;
    let max = 6.0;
    let default = 0.0;
    let format = im_str!("%.2fdB");

    system.main_loop(move |_, ui| {
        Window::new(im_str!("Hello Knob"))
            .size([300.0, 300.0], Condition::FirstUseEver)
            .position([20.0, 20.0], Condition::Appearing)
            .build(ui, || {
                ui.set_window_font_scale(1.0);

                let highlight = ColorSet::new(
                    [0.4, 0.4, 0.8, 1.0],
                    [0.4, 0.4, 0.9, 1.0],
                    [0.5, 0.5, 1.0, 1.0],
                );
                let base = ColorSet::new(
                    [0.4, 0.3, 0.5, 1.0],
                    [0.45, 0.35, 0.55, 1.0],
                    [0.45, 0.35, 0.55, 1.0],
                );
                let lowlight = ColorSet::from([0.0, 0.0, 0.0, 1.0]);

                draw_wiper_knob(
                    &knob_with_drag(
                        ui,
                        im_str!("Knob"),
                        im_str!("Gain"),
                        &mut value,
                        min,
                        max,
                        default,
                        format,
                    ),
                    &base,
                    &highlight,
                    &lowlight,
                );

                ui.next_column();
            });
    });
}
