# Speech to text processing pipeline using coqui stt

Splits mp3s into chunks, converts them to wavs in mono at 16Hz, then processes in parallel using coqui stt. ~2h of audio in ~15min. The more CPU's the faster it will go. set `RAYON_NUM_THREADS` for control

- Requires stt c libraries to be available in the path.
- Requires coqui stt models and scorers
- Variables in `src/constants.rs`

build with `cargo build --release`

```terminal
# Dependencies

python -m pip install coqui-stt-model-manager
python -m pip install stt

# FFMPEG
ffmpeg -i 800.mp3 -acodec pcm_s16le -ac 1 -ar 16000 -aframes 10000 800.wav
ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1:nokey=1
ffmpeg -ss 60 -i input-audio.aac -t 15 -c copy output.aac
ffmpeg -i files/episodes/sn0904.mp3 -acodec pcm_s16le -ac 1 -ar 16000 files/output/sn0904.wav


# DS/STT
pip3 install stt
pip3 install deepspeech

deepspeech --model models/deepspeech-0.9.3-models.pbmm --scorer models/deepspeech-0.9.3-models.scorer --audio sandbox/800.wav --json > out.json

stt --model models/stt-huge-1.4.tflite --scorer models/stt-huge-1.4.scorer --audio sandbox/800.wav --json > out-stt.json

# Deepspeech
curl -LO https://github.com/mozilla/DeepSpeech/releases/download/v0.9.3/deepspeech-0.9.3-models.pbmm
curl -LO https://github.com/mozilla/DeepSpeech/releases/download/v0.9.3/deepspeech-0.9.3-models.scorer

# Coqui STT
https://github.com/coqui-ai/STT/releases/download/v1.4.0/libstt.tflite.Linux.zip
https://coqui.gateway.scarf.sh/english/coqui/v1.0.0-huge-vocab/model.tflite
https://coqui.gateway.scarf.sh/english/coqui/v1.0.0-huge-vocab/huge-vocabulary.scorer

# Assets
https://www.grc.com/sn/sn-904.txt
https://media.grc.com/sn/sn-904.mp3

```


yt-dlp -o - "https://www.youtube.com/watch?v=jm3JFYqvQxw" |

whisper --model base.en --output_dir output --output_format all --word_timestamps True --threads 4

ydl_opts = {
    'format': 'bestaudio/best',
    'postprocessors': [{
        'key': 'FFmpegExtractAudio',
        'preferredcodec': 'mp3',
        'preferredquality': '192',
    }],
    'outtmpl': '%(title)s.%(etx)s',
    'quiet': False
}

with youtube_dl.YoutubeDL(ydl_opts) as ydl:
    ydl.download([url])  # Download into the current working directory

yt-dlp --dump-json "https://www.youtube.com/watch?v=jm3JFYqvQxw" > out.json

ffmpeg -t 1 -i "https://www.youtube.com/watch?v=jm3JFYqvQxw" -acodec pcm_s16le -ac 1 -ar 16000 piped.wav


cat out.json | jq '{ id: .id, title: .title, thumbnail: .thumbnail, description: .description, channel_id: .channel_id, duration: .duration }'
