# SheepRaiderTools
Sheep Raider Tools that allow you to replace text in .exe and .mlt files. Works both on PC and PSX

# Usage
## Decompile
Just drag'n'drop MLT files on mlt_tool.exe icon
## Recompile
```mlt_tool -r replace_table_test.json Lev-01.MLT Lev-02.MLT```
```exe_tool -i SheepD3D.exe pc_exe_config.json strings.json replace_table_test.json```

## Replace table
If you put non-ANSI characters in the .mlt file, they cannot be rendered in the game. For example, if there are some unicode letters in your language, then you'll have to replace with chars that presents in the font file of the game resources.

## Note
The length of the text in .exe files is limited. It can't be bypassed easy. But if you really need very big amount of space, you can replace some phrases in unnecessary languages with 1 letter.
