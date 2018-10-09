# regexfs: **R**usty R**eg**ular **Exp**ressions **F**ile**s**ystem

>     [11:09 PM] Daemojito: I still want to make my regex filesystem.
>     [11:09 PM] zekesonxx: regex filesystem?
>     [11:11 PM] Daemojito: Regfs hahafunny/ -i *.png mount/
>     [11:11 PM] Daemojito: Or something. I'm actually shit with regex.

Presenting, for your consideration, a buggy as fuck FUSE filesystem.

Usage:
```
$ regexfs <base folder> <regular expression> <mountpoint>
```

Example:
```
$ regexfs hahafunny/ '(?i:\.png)' mount/
```

Creates a mountpoint at `mount` containing all files containing ".png" (case insensitive) from the `hahafunny` folder.

## Known Issues
Most of the code. Really, the core functionality of this software is an issue in of itself.

## License
MIT. See `LICENSE`.