use nih_plug::nih_error;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::PolySynthParams;

#[derive(Lens)]
struct Data {
    params: Arc<PolySynthParams>,
}

impl Model for Data {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (500, 400))
}

pub(crate) fn create(
    params: Arc<PolySynthParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        if let Err(err) = cx.add_stylesheet(include_style!("src/theme.css")) {
            nih_error!("Failed to load stylesheet: {err:?}")
        }

        Data {
            params: params.clone(),
        }
        .build(cx);

        VStack::new(cx, |cx| {

            VStack::new(cx, |cx| {
                Label::new(cx, "Kyrim's PolySynth")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_weight(FontWeightKeyword::Thin)
                    .font_size(20.0);
            });

            VStack::new(cx, |cx| {
                Label::new(cx, "Attack");
                ParamSlider::new(cx, Data::params, |params| &params.attack);
            });

            VStack::new(cx, |cx| {
                Label::new(cx, "Decay");
                ParamSlider::new(cx, Data::params, |params| &params.decay);
            });

            VStack::new(cx, |cx| {
                Label::new(cx, "Sustain");
                ParamSlider::new(cx, Data::params, |params| &params.sustain);
            });

            VStack::new(cx, |cx| {
                Label::new(cx, "Release");
                ParamSlider::new(cx, Data::params, |params| &params.release);
            });

            VStack::new(cx, |cx| {
                Label::new(cx, "Glide");
                ParamSlider::new(cx, Data::params, |params| &params.glide);
            });
        })
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0))
        .top(Units::Pixels(40.0))
        .bottom(Units::Pixels(40.0));

        // ResizeHandle::new(cx);
    })
}