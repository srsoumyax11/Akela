//! Silero VAD — ONNX Specialist VAD
//!
//! This is the "Specialist" in the two-stage VAD pipeline.
//! It uses the Silero VAD ONNX model to verify if a segment truly
//! contains speech after the fast WebRTC "scout" triggers.

use anyhow::{Context, Result};
use ort::session::Session;
use ort::value::Value;
use std::path::Path;
use tracing::{info, warn};

pub struct SileroVad {
    session: Session,
}

impl SileroVad {
    /// Initialize the Silero VAD session.
    pub fn new(model_path: &Path) -> Result<Self> {
        info!("Loading Silero VAD from: {}", model_path.display());

        let session = Session::builder()
            .map_err(|e| anyhow::anyhow!("Failed to create SessionBuilder: {:?}", e))?
            .with_intra_threads(1)
            .map_err(|e| anyhow::anyhow!("Failed to set threads: {:?}", e))?
            .commit_from_file(model_path)
            .context("Failed to load Silero VAD model")?;

        info!("Silero VAD loaded successfully");

        Ok(Self { session })
    }

    /// Check if a 16kHz mono f32 buffer contains speech.
    pub fn is_speech(&self, samples: &[f32]) -> Result<bool> {
        let batch_size = 1;
        let samples_count = samples.len();
        
        let input_array = ndarray::Array2::from_shape_vec(
            (batch_size, samples_count),
            samples.to_vec(),
        )?;
        
        let sr_array = ndarray::Array1::from_vec(vec![16000i64]);
        
        // Initial state is zeros [2, 1, 64]
        let state_array = ndarray::Array3::<f32>::zeros((2, 1, 64));

        // Convert ndarray to Value
        let input_value = Value::from_array(input_array)
            .map_err(|e| anyhow::anyhow!("Failed to create input value: {:?}", e))?;
        let sr_value = Value::from_array(sr_array)
            .map_err(|e| anyhow::anyhow!("Failed to create sr value: {:?}", e))?;
        let state_value = Value::from_array(state_array)
            .map_err(|e| anyhow::anyhow!("Failed to create state value: {:?}", e))?;

        let outputs = self.session.run(ort::inputs![
            "input" => input_value,
            "sr" => sr_value,
            "state" => state_value
        ].map_err(|e| anyhow::anyhow!("Failed to prepare inputs: {:?}", e))?)
        .map_err(|e| anyhow::anyhow!("Failed to run session: {:?}", e))?;

        // Extract probability
        let output_value = outputs.get("output").context("Missing output")?;
        let (_shape, output_slice) = output_value.try_extract_tensor::<f32>()
            .map_err(|e| anyhow::anyhow!("Failed to extract output: {:?}", e))?;
        let probability = output_slice[0];

        // Threshold of 0.5
        Ok(probability > 0.5)
    }
}
