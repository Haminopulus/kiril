<div align="center">
  <img src="./images/banner.png" width="426" height="280" />
  <h1>kiril</h1>
</div>

A simple lyric playing utility, that automatically:
- syncs with your active media player via [MPRIS](https://specifications.freedesktop.org/mpris/latest/)
- locates the `.lrc` file corresponding to your currently playing song

## Features

## TODOs
> [!WARNING]
> Support for MPRIS' features is not fully realized by all players, some issues I have encountered and might fix are:

- **MPV:** [mpv-mpris](https://github.com/hoyon/mpv-mpris) does not cache cover arts and instead sends the entire File encoded in base64

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
