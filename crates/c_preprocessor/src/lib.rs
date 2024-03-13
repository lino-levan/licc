use std::{fs::read_to_string, path::PathBuf};

use c_ast::Module;

// enum IncludeType {
//     BuiltIn,
//     Local,
// }

pub fn preprocess(input: PathBuf) -> Module  {
    // let library_search_paths = match OS {
    //     "macos" => vec![PathBuf::from("/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include"), PathBuf::from("/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/Kernel.framework/Headers")],
    //     _ => Vec::new()
    // };
    // TODO(lino-levan): Read C_INCLUDE_PATH to add more library search paths

    let content = read_to_string(input).unwrap();

    c_parser::parse(content)
}
