#Simple Notes 
## Build
```
git clone https://github.com/internetghost0/terminal-notes && cd terminal-notes

# Change path and editor !!
vim src/main.rs

cargo build --release
cd target/release
cp notes ~/.local/bin/

```
## Example
```
$ notes
~ !important
~ note00
~ note01
~ .garbage

$ notes note01
hello world :)

$ notes important
set a star

$ notes w .garbage

$ notes help
Usage: ./notes [FLAG] <note>
Flags:
        h, help
        l, list
        r, read     <note>
        ra, readall <note>
        w, write    <note>
        mv, rename  <note old> <note new>
        rm, remove  <note>
        shred       <note>

```

## Color
If your terminal does not support color output, then in 'print_notes():88 line' comment
```
println!("~ {}{}{}", cyan, note, reset);
```
and uncomment
```
println!("~ {}", note);
```
