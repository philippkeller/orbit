```
   ____  ____  ____  __________
  / __ \/ __ \/ __ )/  _/_  __/
 / / / / /_/ / __  |/ /  / /   
/ /_/ / _, _/ /_/ // /  / /    
\____/_/ |_/_____/___/ /_/     
```

# ◈ Orbit

A beautiful **music player for your terminal** that makes a local library feel alive. Only the good stuff from streaming services, but the music is yours. 
Organise tracks into **buckets** you dump into the queue, sculpt the sound with a real
**10-band equalizer**, drift off in a full-screen **zen mode**, and let Orbit spin up a
**radio of similar songs** — recommendations from listening to the audio itself, fully
offline, no accounts, nothing leaves your machine. Plays MP3, FLAC, WAV, OGG, M4A/MP4,
and AAC.

Built in Rust with [ratatui](https://ratatui.rs) and
[rodio](https://github.com/RustAudio/rodio). Runs on macOS, Linux, and Windows, with
hardware media-key and system Now Playing integration.

## Install

From the repo (any Rust toolchain):

```sh
cargo install --git https://github.com/sihooleebd/orbit
```

Or from a local clone — re-run with `--force` to update:

```sh
cargo install --path . --root ~/.local
```

**Linux** also needs ALSA + D-Bus development packages:

```sh
sudo apt install libasound2-dev libdbus-1-dev pkg-config        # Debian/Ubuntu
sudo dnf install alsa-lib-devel dbus-devel pkgconf-pkg-config   # Fedora
```

## Run

```sh
cargo run --release
```

On first launch Orbit adopts your **Music** folder if it exists; press `A` to manage
library folders and `R` to rescan. Config, buckets, and the library cache live under
your platform data dir (`~/Library/Application Support/orbit` on macOS).

### Remote control (fork)

A running Orbit instance listens on a Unix domain socket
(`~/Library/Application Support/orbit/orbit.sock` on macOS). Use this from Karabiner,
shell scripts, or another terminal — Orbit does not need keyboard focus.

```sh
orbit --remote delete-current        # trash the playing track, play next
orbit --remote notify-now-playing    # macOS notification for now playing
orbit --remote seek-forward          # skip +5 seconds (same as →)
orbit --remote seek-backward         # skip −5 seconds (same as ←)
```

Restart Orbit after upgrading so the running instance picks up new remote commands.

## Changes from upstream

Additional behaviour on top of stock Orbit (original by Benjamin Lee):

- **Play from here** — in folder browse mode, `Enter` on a track replaces the queue
  with that track and every following track in the current view (displayed order), then
  starts playback immediately. Upstream appended a single track instead.
- **Delete playing file (`X`)** — permanently delete the currently playing track from
  disk after confirmation. Removes it from the queue and library cache without a
  rescan, then plays the next queued track (or stops cleanly).
- **Remote CLI** — `orbit --remote …` talks to a running instance over a Unix socket.
  - `delete-current` — move the playing file to the Trash (no confirmation), update
    queue/library, play next.
  - `notify-now-playing` — show a macOS desktop notification with artist, title, and
    album (no-op when nothing is playing).
  - `seek-forward` / `seek-backward` — jump ±5 seconds (same as `→` / `←` in the UI).
- **macOS sleep/wake recovery** — after closing the laptop lid, Orbit detects a
  frozen playback position and reopens the audio output automatically (~3 s),
  resuming the current track without losing the queue or UI state. If recovery
  fails, press `Space` to retry.
- **Session restore** — quitting with `q` saves the queue, current track (and
  position), library folder/search, and pane focus to `session.json`. The next
  launch picks up where you left off.

## Screenshots

The three-pane overview — library, buckets, and the queue:

<p align="center"><img src="assets/overview.png" width="760" alt="Overview"></p>

Zen mode (`z`) — full-screen player with two visualizers you flip between with `v`:

<table>
  <tr>
    <td width="50%"><img src="assets/zen-cassette.png" width="100%" alt="Zen — cassette"></td>
    <td width="50%"><img src="assets/zen-spectrum.png" width="100%" alt="Zen — spectrum"></td>
  </tr>
</table>

The equalizer (`e`) and the About card (`i`):

<table>
  <tr>
    <td width="50%"><img src="assets/equalizer.png" width="100%" alt="Equalizer"></td>
    <td width="50%"><img src="assets/about.png" width="100%" alt="About"></td>
  </tr>
</table>

### Themes

Ten built-in palettes — open **Settings** (`,`) → **Theme** for a live picker:

<table width="100%">
  <tr>
    <td align="center" width="10%"><img src="assets/themes/synthwave.png" width="100%"></td>
    <td align="center" width="10%"><img src="assets/themes/nord.png" width="100%"></td>
    <td align="center" width="10%"><img src="assets/themes/matrix.png" width="100%"></td>
    <td align="center" width="10%"><img src="assets/themes/solarized.png" width="100%"></td>
    <td align="center" width="10%"><img src="assets/themes/ember.png" width="100%"></td>
    <td align="center" width="10%"><img src="assets/themes/dracula.png" width="100%"></td>
    <td align="center" width="10%"><img src="assets/themes/tokyo-night.png" width="100%"></td>
    <td align="center" width="10%"><img src="assets/themes/catppuccin.png" width="100%"></td>
    <td align="center" width="10%"><img src="assets/themes/gruvbox.png" width="100%"></td>
    <td align="center" width="10%"><img src="assets/themes/rose-pine.png" width="100%"></td>
  </tr>
  <tr>
    <td align="center" width="10%"><sub>Synthwave</sub></td>
    <td align="center" width="10%"><sub>Nord</sub></td>
    <td align="center" width="10%"><sub>Matrix</sub></td>
    <td align="center" width="10%"><sub>Solarized</sub></td>
    <td align="center" width="10%"><sub>Ember</sub></td>
    <td align="center" width="10%"><sub>Dracula</sub></td>
    <td align="center" width="10%"><sub>Tokyo Night</sub></td>
    <td align="center" width="10%"><sub>Catppuccin</sub></td>
    <td align="center" width="10%"><sub>Gruvbox</sub></td>
    <td align="center" width="10%"><sub>Rosé Pine</sub></td>
  </tr>
</table>

## Keys

**Navigate** — `Tab` panes · `↑↓`/`j k` move · `Enter` open folder / play from here · `⌫` up · `/` search · `g`/`G` top/bottom

**Playback** — `Space` pause · `n`/`p` next/prev · `←→` seek · `+`/`-` volume · `s` shuffle · `r` repeat

**Buckets** — `b` new · `S` save queue · `a` add track · `o` open/edit · `m` radio (similar) · `d` dump · `x` delete/remove · `X` delete playing file · `c` clear queue

**Player & more** — `A` folders · `R` rescan · `D` download (yt-dlp) · `e` EQ · `E` EQ on/off · `z` zen · `v` visualizer · `,` settings · `i` about · `?` help · `q` quit

## Features

- **Buckets** — name playlists and `d`-dump them into the queue. `o` opens one to
  play, remove, reorder, or rename tracks; `S` saves the current queue as a bucket;
  each gets its own accent colour.

- **Smart buckets** — auto-filled *Recently Added*, *Most Played*, and *Recently
  Played*, built from play stats Orbit keeps as you listen.

- **Radio / recommendations** — Orbit analyses your library in the background
  (MFCC timbre fingerprints + spectral features) and suggests acoustically similar
  music — **100% offline, no accounts**. A `≈ Radio` smart bucket fills itself from
  what you've been playing, and `m` starts a radio queue from the selected track.
  Settings let you scope it to your whole **library** or just the **current folder**.

- **Folder browsing** — the library navigates by folder (`Enter` / `⌫`); `Enter` on a
  track plays from here (replaces the queue with that track and the rest of the current
  view); `/` searches everything; `A` opens a built-in folder picker to add or remove
  roots.

- **Download** (`D`) — paste a URL, pick a download root and folder name, and Orbit
  fetches the audio as mp3 in the background via [yt-dlp](https://github.com/yt-dlp/yt-dlp)
  (with embedded metadata + cover art), then auto-tracks the folder and rescans. Requires
  `yt-dlp` on your `PATH`; the key is inert without it.

- **Equalizer** (`e`) — a real RBJ-biquad 10-band EQ drawn FabFilter-style: a response
  line over a live spectrum, with five presets and a pre-amp. Turns on the moment you
  touch it; settings persist.

- **Zen mode** (`z`) — full-screen player with synced `.lrc` lyrics and two
  visualizers (`v`): a live audio spectrum or an animated cassette deck.

- **Settings** (`,`) — one hub for the **equalizer**, the **theme** picker (ten
  palettes, live preview, saved), the zen visualizer, a **sleep timer**
  (15/30/45/60 min or end-of-track, with a fade-out), the **radio scope**, and a
  footer-hints toggle.

- **OS integration** — hardware media keys and the system Now Playing panel
  (Control Center / MPRIS / SMTC). `orbit --remote notify-now-playing` posts a macOS
  notification for the current track (handy with Karabiner or other launchers).

- **Safe & resilient** — confirmation prompts before destructive actions, and
  event-driven recovery if the audio output device disappears or changes
  mid-song. On macOS and Linux, a position-stall heuristic also recovers after
  sleep/wake when the OS does not report device loss; press `Space` to retry
  manually if auto-recovery gives up.

## License

[MIT](LICENSE) © 2026 Benjamin Lee
