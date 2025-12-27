# Digger Decoder

![Digging](https://github.com/chrishulbert/digger-decoder/blob/main/readme/digging.png?raw=true)

This is a tool for decoding all the images/animations/maps from classic DOS Lemmings :)

## How to use

![Mining](https://github.com/chrishulbert/digger-decoder/blob/main/readme/mining.png?raw=true)

* Check out the repo
* Run 'make run' to extract all the files

## Animations

![Bricklaying](https://github.com/chrishulbert/digger-decoder/blob/main/readme/bricklaying.png?raw=true)

This outputs uncompressed PNG / APNG files. If that concerns you, you can compress them like so:

    brew install apngasm
    brew install pngquant
    make compress

Animations are quite small, to embiggen them you can:

    ffmpeg -i walking.png -vf "scale=iw*4:ih*4:flags=neighbor" -plays 0 walking.apng

## See also

![Dopefish](https://github.com/chrishulbert/dopefish-decoder/raw/main/Dopefish.png?raw=true)

If you love Commander Keen, please check out my [Dopefish decoder](https://github.com/chrishulbert/dopefish-decoder) too :)
