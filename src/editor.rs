use std::sync::Arc;

use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};

use nih_plug::prelude::*;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;

use crate::NihSamplerParams;

#[derive(Lens)]
struct Data {
    params: Arc<NihSamplerParams>,
    debug: String,
}

impl Model for Data {}

pub fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (700, 700))
}

pub fn create(
    params: Arc<NihSamplerParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        // cx.add_theme(include_str!("theme.css"));
        // cx.add_fonts_mem(&[include_bytes!("./BebasNeue-Regular.ttf")]);

        Data {
            params: params.clone(),
            debug: "nothing".into(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Nih Downmixer")
                .font_size(30.0)
                .height(Pixels(50.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0));

            HStack::new(cx, |cx| {
                Label::new(cx, "1/2");
                ParamSlider::new(cx, Data::params, |params| &params.mix);
                Label::new(cx, "3/4");
            });

            // PeakMeter::new(
            //     cx,
            //     Data::peak_meter
            //         .map(|peak_meter| util::gain_to_db(peak_meter.load(Ordering::Relaxed))),
            //     Some(Duration::from_millis(600)),
            // )
            // This is how adding padding works in vizia
            // .top(Pixels(10.0));
        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));

        ResizeHandle::new(cx);
    })
}
