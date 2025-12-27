# Digger Decoder

![Digging](https://github.com/chrishulbert/digger-decoder/raw/main/readme/digging.png?raw=true)

This is a tool for decoding all the images/animations/maps from classic DOS Lemmings :)

## How to use

![Mining](https://github.com/chrishulbert/digger-decoder/raw/main/readme/mining.png?raw=true)

* Check out the repo
* Run 'make run' to extract all the files from the OG lemmings
* You can also run other lemmings:
    * make run-ohnomore
    * make run-christmas91
    * make run-christmas92
    * make run-holidays93
    * make run-holidays94

## Animations

![Bricklaying](https://github.com/chrishulbert/digger-decoder/raw/main/readme/bricklaying.png?raw=true)

This outputs uncompressed PNG / APNG files. If that concerns you, you can compress them like so:

    brew install apngasm
    brew install pngquant
    make compress

Animations are quite small, to embiggen them you can:

    ffmpeg -i walking.png -vf "scale=iw*4:ih*4:flags=neighbor" -plays 0 walking.apng

## See also

![Dopefish](https://github.com/chrishulbert/dopefish-decoder/raw/main/Dopefish.png?raw=true)

If you love Commander Keen, please check out my [Dopefish decoder](https://github.com/chrishulbert/dopefish-decoder) too :)

## Docs

![Kessel](https://github.com/chrishulbert/digger-decoder/raw/main/readme/thesteelminesofkessel.png?raw=true)
![Down along up](https://github.com/chrishulbert/digger-decoder/raw/main/readme/downalongup.png?raw=true)
![You need bashers](https://github.com/chrishulbert/digger-decoder/raw/main/readme/youneedbashers.png?raw=true)
![Menacing](https://github.com/chrishulbert/digger-decoder/raw/main/readme/menacing.png?raw=true)
![Just a minute](https://github.com/chrishulbert/digger-decoder/raw/main/readme/justaminute.png?raw=true)
![Only floaters](https://github.com/chrishulbert/digger-decoder/raw/main/readme/onlyfloaters.png?raw=true)

Thanks to [camanis.net](https://www.camanis.net/lemmings/files/docs/) for documenting the file formats.
