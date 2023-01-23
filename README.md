
// ffmpeg -i 800.mp3 -acodec pcm_s16le -ac 1 -ar 16000 -aframes 10000 800.wav
// deepspeech --model models/deepspeech-0.9.3-models.pbmm --scorer models/deepspeech-0.9.3-models.scorer --audio sandbox/800.wav --json > out.json

// stt --model models/stt-huge-1.4.tflite --scorer models/stt-huge-1.4.scorer --audio sandbox/800.wav --json > out-stt.json

// https://media.grc.com/sn/sn-905-lq.mp3
// ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1:nokey=1

// ffmpeg -ss 60 -i input-audio.aac -t 15 -c copy output.aac
