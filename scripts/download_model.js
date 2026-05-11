import fs from 'fs';
import path from 'path';
import { Readable } from 'stream';
import { finished } from 'stream/promises';

const MODEL_URL = 'https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin';
const MODEL_DIR = path.join(process.cwd(), 'models');
const MODEL_PATH = path.join(MODEL_DIR, 'ggml-base.en.bin');

async function downloadModel() {
  if (fs.existsSync(MODEL_PATH)) {
    console.log('✅ Whisper model already exists at:', MODEL_PATH);
    return;
  }

  if (!fs.existsSync(MODEL_DIR)) {
    fs.mkdirSync(MODEL_DIR, { recursive: true });
  }

  console.log('📥 Downloading Whisper model (ggml-base.en.bin) ~140MB...');
  console.log('🔗 URL: ' + MODEL_URL);
  
  const response = await fetch(MODEL_URL);
  if (!response.ok) {
     // Handle redirects if fetch doesn't automatically (it should)
     throw new Error(`Failed to download model: ${response.status} ${response.statusText}`);
  }

  const fileStream = fs.createWriteStream(MODEL_PATH);
  
  // Use Readable.fromWeb for Node 18+ / Bun compatibility
  const body = Readable.fromWeb(response.body);
  await finished(body.pipe(fileStream));

  console.log('✅ Model download complete! Saved to:', MODEL_PATH);
}

downloadModel().catch(err => {
  console.error('❌ Error downloading model:', err.message);
  // We don't exit with 1 because we don't want to break the whole build 
  // if the user wants to provide the model manually.
  // process.exit(1); 
});
