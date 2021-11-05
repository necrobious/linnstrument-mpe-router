# linnstrument-mpe-router
Route LinnStrument splits to two separate synths, that use the same MIDI channels for MPE (e.g OB-6 &amp; P6)

# Description
A Linnstrument, configured for MPE on both splits, will use channels 1 and 16 for global MPE events, and channels 2-15 for per-note events.
Integrating the LinnStrument with two synths, Left and Right, may be impossible, if the two target synths receive MPE events on the same channel.
An example of this, is the Sequential OB-6 and Prophet 6 synths; both expect to operate with MPE using channels 1-7.


linnstrument-mpe-router remaps incomming ALSA MIDI event channels from a LinnStrument to the same MIDI channels 1-8, using the following:

| Source Channel | Destination | Destination Channel |
| -------------- | ----------- | ------------------- |
|       1        |    LEFT     |          1          |
|       2        |    LEFT     |          2          |
|       3        |    LEFT     |          3          |
|       4        |    LEFT     |          4          |
|       5        |    LEFT     |          5          |
|       6        |    LEFT     |          6          |
|       7        |    LEFT     |          7          |
|       8        |    LEFT     |          8          |
|       9        |    RIGHT    |          8          |
|      10        |    RIGHT    |          7          |
|      11        |    RIGHT    |          6          |
|      12        |    RIGHT    |          5          |
|      13        |    RIGHT    |          4          |
|      14        |    RIGHT    |          3          |
|      15        |    RIGHT    |          2          |
|      16        |    RIGHT    |          1          |

# Building
```
cargo build --release
```

# Running
```
./target/release/linnstrument-mpe-router -i 'LinnStrument MIDI:LinnStrument MIDI MIDI 1' -l 'OB-6 Module:OB-6 Module MIDI 1' -r 'Prophet 6 Module:Prophet 6 Module MIDI 1'
```

