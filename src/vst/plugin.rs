use vst2::buffer::AudioBuffer;
use vst2::plugin::{Category, Plugin, Info, HostCallback};
use vst2::editor::Editor;
use vst2::host::Host;

use simplelog::*;
use std::fs::File;

use window::window::ConrodWindow;

use app::config::*;

// #[derive(Default)]
pub struct VSTPlugin {
    threshold: f32,
    gain: f32,
    pub window: Option<ConrodWindow>,
    pub app: AppConfig,
}

impl Default for VSTPlugin {
    fn default() -> VSTPlugin {
        use log_panics;
        log_panics::init;
        let _ = CombinedLogger::init(
            vec![
                WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create("/tmp/simplesynth.log").unwrap()),
            ]
        );
        VSTPlugin {
            threshold: 1.0, // VST parameters are always 0.0 to 1.0
            gain: 1.0,
            window: None,
            app: AppConfig::default(),
        }
    }
}

impl Plugin for VSTPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "DDConrod".to_string(),
            vendor: "DeathDisco".to_string(),
            unique_id: 7790,
            category: Category::Effect,

            inputs: 2,
            outputs: 2,
            parameters: 2,

            ..Info::default()
        }
    }

    fn can_be_automated(&self, index: i32) -> bool { true }

    fn get_editor(&mut self) -> Option<&mut Editor> {
        Some(self)
    }

    fn get_parameter(&self, index: i32) -> f32 {
        self.app.params.params[index as usize].value
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        self.app.params.params[index as usize].value = value.max(0.01);
    }

    fn get_parameter_name(&self, index: i32) -> String {
        self.app.params.params[index as usize].name.clone()
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            // Convert to a percentage
            0 => format!("{}", self.threshold * 100.0),
            1 => format!("{}", self.gain * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "%".to_string(),
            1 => "%".to_string(),
            _ => "".to_string(),
        }
    }

    fn process(&mut self, buffer: AudioBuffer<f32>) {
        // Split out the input and output buffers into two vectors
        let (inputs, outputs) = buffer.split();

        // For each buffer, transform the samples
        for (input_buffer, output_buffer) in inputs.iter().zip(outputs) {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {

                if *input_sample >= 0.0 {
                    *output_sample = input_sample.min(self.threshold) / self.threshold * self.gain;
                }
                else {
                    *output_sample = input_sample.max(-self.threshold) / self.threshold * self.gain;
                }

            }
        }
    }
}