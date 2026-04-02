<div align="center">
  <img src="./images/banner.png" width="426" height="280" />
  <h1>✨kiril✨</h1>
</div>

A simple lyric playing utility, that automatically:
- syncs with your active media player via [MPRIS](https://specifications.freedesktop.org/mpris/latest/) (using the [mpris crate](https://docs.rs/mpris/latest/mpris/))
- locates the [LRC](https://en.wikipedia.org/wiki/LRC_(file_format))-file corresponding to your currently playing song
- parses said `.lrc` file and displays the lines/words that are currently being sung

## How it works
Once kiril is started it begins periodically checking for an active MPRIS-player. Once one is found the url of the currently playing track is queried.
Lets say you are currently listening to:
```
/foo/bar/trackname.flac
```
Then kiril will search in `/foo/bar/` for the corresponding LRC-file called `trackname.lrc`. If it could not be found there, for each dir in `/foo/bar/` kiril will recursively search for the LRC file again. Currently kiril only does this once, for direct subdirectories but i am planning to make that customizable (as well as adding custom directories, see TODOs)

Once a matching file is found, kiril parses it to extract timestamps for all the lines in the track (also supports [word-by-word](https://en.wikipedia.org/wiki/LRC_(file_format)#A2_extension_.28Enhanced_LRC_format.29) synced lyrics) and prints them to the terminal when they are sung. Currently lyrics can be printed:

- in plain text (Just the words being sung with no additional information)
![plain text example video](graphics/plaintext.mp4?raw=true "")
- in JSON format (containing information like, next/prev lines/words, currently sung line/word and cover art url) which I found helpful for making widgets using [eww](https://github.com/elkowar/eww)
![eww example video](graphics/eww.mp4?raw=true "")


## Tested Players
> [!NOTE]
> I did not write or test this for streaming services such as Spotify or Apple Music (Which do also have their own lyric-viewer implementations). There are no plans to change that any time soon. This is intended for local file playback with locally stored `.lrc` files only!

Support for MPRIS' features is not fully realized by all players, here are the results for all the players I have tested so far:

- **[cmus](https://github.com/cmus/cmus):** provides neither song-url, nor cover-art-url ([relevant PR, closed](https://github.com/cmus/cmus/pull/1009))
- **[VLC](https://www.videolan.org/):** confirmed working ✅
- **[mpv](https://mpv.io/):** confirmed working ✅ (using [mpv-mpris](https://github.com/hoyon/mpv-mpris))
- **[Elisa](https://github.com/KDE/elisa):** confirmed working ✅
- **[Lollypop](https://gitlab.gnome.org/World/lollypop):** confirmed working ✅
- **[RythmBox](https://gitlab.gnome.org/GNOME/rhythmbox):** confirmed working ✅
- **[QuodLibet](https://github.com/quodlibet/quodlibet):** confirmed working ✅

## Installation

<li> First clone the Repository

    $ git clone https://github.com/Haminopulus/kiril.git
    $ cd kiril

</li>
<li> Then build:

    $ cargo build --release
The binary can now be found under `./kiril/target/release/kiril`
</li>
<li> You might need to make it executable

    $ cd ./kiril/target/release/
    $ chmod +x ./kiril
</li>
<li> Run it:

    $ ./kiril
</li>
<li> Check out the help command for possible args:

    $ ./kiril --help
</li>

## TODOs
- Support for custom lyric directories
- Support for other metadata (artist, title, etc.)
- Support for tags within `.lrc` files (especially offset, the other metadata can be extracted via MPRIS usually)
- a commandline arg that sets how many recursion layers kiril is allowed, when searching the LRC file


