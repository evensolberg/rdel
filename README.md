# rdel

Recursively delete files.

## Usage

`rdel [FLAGS] [FILE(S)]...`

Example: `rdel -r -p images/**/*.gif images/**/*.tif`

This will list all the files targeted for deletion along with a count of the number of files.

### Flags

|Short Form|Long Form|Description|
|:----|:---|:----------|
`-d`|`--debug`|Output debug information as we go. Supply it twice for trace-level logs.
`-o`|`--detail-off`|Don't export detailed information about each file processed.
`-r`|`--dry-run`|Iterate through the files and produce output without actually deleting anything.
`-h`|`--help`|Prints help information
`-p`|`--print-summary`|Print summary detail.
`-q`|`--quiet`|Don't produce any output except errors while working.
`-V`|`--version`|Prints version information

### Arguments

|Argument|Description|
|:-------|:----------|
`<FILE(S)>...`|One or more file(s) to process. Wildcards and multiple files (e.g. `2019*.pdf 2020*.pdf`) are supported. Use `**` glob to recurse (i.e. `**/*.pdf`).<br>**Note: Case sensitive.**

## Notes

Currently, using `zsh` on the Mac, the program exits with an error if one of the `<FILE>` arguments isn't found (ie. `*.jpg *.jpeg *.png` - `*.jpeg` not found). This is due to how this is handled in the shell.

You can work around this by using the following command: `setopt +o NO_MATCH`
