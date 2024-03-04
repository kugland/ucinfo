# ucinfo

**This program is in the early stages of development. Nothing works yet.**

ucinfo - show information about Unicode characters on the command line.

## Usage

```console
$ ucinfo --help
Show information about Unicode characters.

ucinfo [OPTIONS] [CHARACTER...]

ucinfo [OPTIONS] -S [SEARCH]

CHARACTER can be either a string, which will show information for each
character in the string, or a Unicode code point in the form U+XXXX.

Options:
    -S, --search STRING  Search for characters by Unicode name
    -n, --no-draw        Do not draw character glyphs
    -a, --all            Show all available information
    -j, --json           Output information in JSON format
    -H, --html           Output information in HTML format
    -h, --help           Show this help message and exit
    -V, --version        Show version information and exit
```

## Examples

Not all information is correct in the following examples, they are just for
illustration purposes.

```console
$ ucinfo 'Á'

                U+0041 'Á'
     ▄▄▀▀       LATIN CAPITAL LETTER A WITH ACUTE

     ▄▀▀▄       Unicode Version:  1.1 (June 1993)
    ▄▀  ▀▄                Block:  Latin-1 Supplement (U+0080–U+00FF)
    █▄▄▄▄█                Plane:  Basic Multilingual Plane (U+0000–U+FFFF)
    █    █               Script:  Latin (Latn)
    █    █             Category:  Uppercase Letter (Lu)

         UTF-8:  C3 81    UTF-16:  00 C1    UTF-32:  00 00 00 C1


    Lowercase:  U+00E1 'á'
Decomposition:  U+0041 (A) + U+0301 (´)
      Spacing:  Yes
    Direction:  Left to Right (L)

  Other representations:
     HTML:  &Aacute;  &#193;  &#xC1;
    LaTeX:  \'{A}  \textAacute{}


Charsets that contain this character:
    ISO-8859-1 (0xC1), Windows-1252 (0xC1), MacRoman (0xC1),
    IBM437 (0xB5), IBM850 (0xB5), IBM852 (0xC1), IBM855 (0xC1)
```
