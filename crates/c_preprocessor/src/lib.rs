use std::{env::consts::OS, fs::read_to_string, path::PathBuf};

enum IncludeType {
    BuiltIn,
    Local,
}

pub fn preprocess(input: PathBuf) -> String {
    let library_search_paths = match OS {
        "macos" => vec![PathBuf::from("/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include"), PathBuf::from("/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/Kernel.framework/Headers")],
        _ => Vec::new()
    };
    // TODO(lino-levan): Read C_INCLUDE_PATH to add more library search paths

    let mut result = String::from("");

    let content = read_to_string(input).unwrap();
    let lines = content.split("\n");

    for line in lines {
        let trimmed = line.trim();
        if !trimmed.starts_with("#") {
            result += line;
            result += "\n";
            continue;
        }

        let directive = trimmed[1..].split_whitespace().next().unwrap();

        match directive {
            "include" => {
                let rest = &trimmed[directive.len() + 2..];
                let include_type = match rest.chars().nth(0) {
                    Some('<') => IncludeType::BuiltIn,
                    Some('"') => IncludeType::Local,
                    _ => panic!("Unknown include type!"),
                };
                let rest = &rest[1..rest.len() - 1];

                let resolved_path = match include_type {
                    IncludeType::BuiltIn => {
                        let mut result = None;
                        for search_path in &library_search_paths {
                            let potential_path = search_path.join(rest);
                            if potential_path.exists() {
                                result = Some(potential_path);
                                break;
                            }
                        }
                        result
                    }
                    IncludeType::Local => {
                        panic!("Not implemented");
                    }
                };

                println!("{}", rest);
                result += preprocess(resolved_path.unwrap()).as_str();
            }
            "ifdef" => (),
            "ifndef" => (),
            "define" => (),
            "undef" => (),
            "if" => (),
            "elif" => (),
            "else" => (),
            "endif" => (),
            "warning" => (),
            "error" => (),
            _ => panic!("Unknown directive! {}", directive),
        }
    }

    result
}
