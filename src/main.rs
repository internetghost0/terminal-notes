use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

// change it
static PATH: &str = "/home/USER/.local/share/notes"; // all notes will locate in this folder
static EDITOR: &str = "vim";
//

fn usage() {
    println!("Usage: ./notes [FLAG] <note>");
    println!("Flags:");
    println!("\th, help");
    println!("\tl, list");
    println!("\tr, read     <note>");
    println!("\tra, readall <note>");
    println!("\tw, write    <note>");
    println!("\tmv, rename  <note old> <note new>");
    println!("\trm, remove  <note>");
    println!("\tshred       <note>")
}

fn main() {
    if !Path::new(PATH).exists() {
        eprintln!("Folder `{}` does not exists.", PATH);
        exit(-1);
    }
    let mut argv = env::args().collect::<Vec<_>>();
    argv.remove(0);
    let argc = argv.len();
    if argc == 0 {
        print_notes();
        return;
    } else if argc == 1 {
        match argv[0].as_str() {
            "help" | "h" => usage(),
            "--help" | "-h" => usage(),
            "list" | "l" => print_notes(),
            "readall" | "ra" => read_all_notes(),
            _ => {
                read_note(&argv[0]);
            }
        }
    } else if argc == 2{
        match argv[0].as_str() {
            "write" | "w" => write_note(&argv[1]),
            "read" | "r" => read_note(&argv[1]),
            "remove" | "rm" => delete_note(&argv[1]), // just delete file
            _ => {
                eprintln!("ERROR: Unknown flag `{}`", argv[0].as_str());
                eprintln!("try: notes help");
            }
        }
    } else if argc == 3 {
        match argv[0].as_str() {
            "rename" | "mv" => rename_note(&argv[1], &argv[2]),
            _ => {
                eprintln!("ERROR: Unknown flag `{}`", argv[0].as_str());
                eprintln!("try: notes help");
            }
        }
    } else {
        usage();
    }
}

fn list_notes() -> Vec<String> {
    let mut notes = fs::read_dir(PATH)
        .unwrap()
        .map(|res| res.map(|e| e.file_name()).unwrap().into_string().unwrap())
        .collect::<Vec<_>>();
    notes.sort();
    return notes;
}

fn print_notes() {
    let cyan = "\x1B[1;36m";
    let reset = "\x1B[0m";
    let notes = list_notes();
    if notes.len() == 0 {
        println!("[empty]");
        return ();
    }
    let p_note = |n : String| {  
        println!("~ {}{}{}", cyan, n, reset);  // print with color
        //println!("~ {}", n);                  // print without color
    };

    let mut notes_high_priority: Vec<String> = vec![];
    let mut notes_normal_priority: Vec<String> = vec![];
    let mut notes_low_priority: Vec<String> = vec![];

    for note in notes {
        if note.starts_with(".") {
            notes_low_priority.push(note);
            continue;
        } else if note.starts_with("!") {
            notes_high_priority.push(note);
        } else {
            notes_normal_priority.push(note);
        }
    }
    for note in notes_high_priority {
        p_note(note);
    }
    for note in notes_normal_priority {
        p_note(note);
    }
    for note in notes_low_priority {
        p_note(note);
    }
}

fn get_note_path(note_name: &str) -> Result<PathBuf, &str> {
    for note in list_notes() {
        if note.replace("!", "") == note_name {
            return Ok(Path::new(PATH).join(Path::file_name(Path::new(&note)).unwrap()));
        }
    }
    return Err("[!] Failed to find the note ");
}

fn read_note(note_name: &str) {
    let path = get_note_path(note_name);
    let path = match path {
        Ok(p) => p,
        Err(_) => {
            eprintln!("[!] Failed to read the note `{}`", note_name);
            return;
        }
    };
    match fs::read_to_string(path) {
        Ok(note) => print!("{}", note),
        _ => eprintln!("[!] Failed to read the note `{}`", note_name),
    };
}

fn read_all_notes() {
    let line = "---------------";
    for note in list_notes() {
        println!("[{}]", note);

        match fs::read_to_string(Path::new(PATH).join(Path::file_name(Path::new(&note)).unwrap())) {
            Ok(text) => print!("{}", text),
            _ => eprintln!("[!] Failed to read the note `{}`", note),
        };

        println!("{}", line);
    }
}

fn write_note(note_name: &str) {
    let path = Path::new(PATH).join(Path::file_name(Path::new(note_name)).unwrap());
    let path = get_note_path(note_name).unwrap_or(path);
    let mut editor = Command::new(EDITOR).arg(path).spawn().unwrap();
    editor.wait().unwrap();
}

fn rename_note(src: &str, dst:&str) {
    let path_src = get_note_path(src);
    if path_src.is_err() {
        eprintln!("[!] `{}` does not exist", src);
        return;
    } 
    let path_src = path_src.unwrap();
    let path_dst = Path::new(PATH).join(Path::file_name(Path::new(dst)).unwrap());

    if Path::new(&path_dst).exists() {
        eprintln!("[!] `{}` alredy exists",  dst);
        exit(-1);
    }
    let res = fs::rename(path_src, path_dst);
    match res {
        Err(e) => eprintln!("{}", e),
        _ => (),
    }
}


fn delete_note(note_name: &str) {
    let path = get_note_path(note_name);
    if path.is_err() {
        eprintln!("[!] Can't find the note `{}`", note_name);
        return;
    }
    let path = path.unwrap();
    match fs::remove_file(path) {
        Ok(_) => return,
        _ => eprintln!("[!] Failed to delete the note `{}`", note_name),
    }
}

fn exit(code: i32) {
    std::process::exit(code);
}
