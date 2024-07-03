use nih_plug_vizia::ViziaState;
use std::sync::Arc;

use nih_plug::prelude::*;
mod editor;
mod playing_sample;

/// Main plugin struct
pub struct NihDownmixer {
    pub params: Arc<NihSamplerParams>,
}

impl Default for NihDownmixer {
    fn default() -> Self {
        Self {
            params: Arc::new(Default::default()),
        }
    }
}

/// Plugin parameters struct
#[derive(Params)]
pub struct NihSamplerParams {
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    #[id = "mix"]
    pub mix: FloatParam,
}

impl Default for NihSamplerParams {
    fn default() -> Self {
        Self {
            editor_state: ViziaState::new(|| (400, 700)),
            mix: FloatParam::new("Mix", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_smoother(SmoothingStyle::Logarithmic(10.0))
                .with_unit("%")
                .with_value_to_string(formatters::v2s_f32_percentage(1))
                .with_string_to_value(formatters::s2v_f32_percentage()),
        }
    }
}

impl Plugin for NihDownmixer {
    const NAME: &'static str = "Nih Downmixer";
    const VENDOR: &'static str = "matidfk";
    const URL: &'static str = "https://youtu.be/dQw4w9WgXcQ";
    const EMAIL: &'static str = "info@example.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        aux_input_ports: &[unsafe { NonZeroU32::new_unchecked(2) }],
        ..AudioIOLayout::const_default()
    }];

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.params.editor_state.clone())
    }

    fn initialize(
        &mut self,
        audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        nih_log!("changed sample rate to {}", buffer_config.sample_rate);

        return true;
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mut main = buffer.iter_samples();
        let mut aux = aux.inputs[0].iter_samples();
        while let Some(mut channel_samples) = main.next() {
            let mix = self.params.mix.smoothed.next();
            if let Some(mut aux_channel_samples) = aux.next() {
                unsafe {
                    *channel_samples.get_unchecked_mut(0) = *channel_samples.get_unchecked_mut(0)
                        * (1.0 - mix)
                        + *aux_channel_samples.get_unchecked_mut(0) * mix;
                    *channel_samples.get_unchecked_mut(1) = *channel_samples.get_unchecked_mut(1)
                        * (1.0 - mix)
                        + *aux_channel_samples.get_unchecked_mut(1) * mix;
                }
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for NihDownmixer {
    const CLAP_ID: &'static str = "com.moist-plugins-gmbh.the-moistest-plugin-ever";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A simple random-selection sampler");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for NihDownmixer {
    const VST3_CLASS_ID: [u8; 16] = *b"NihDownmixerrrrr";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        // Vst3SubCategory::Drum,
        // Vst3SubCategory::Sampler,
        // Vst3SubCategory::Instrument,
    ];
}

nih_export_clap!(NihDownmixer);
nih_export_vst3!(NihDownmixer);
