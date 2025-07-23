set -e

# block common constants (INCLUDE_VERSION, LIBRARY_MINIMUM), standard C types ending with '_t', and few missing structs (DiskFont, DiskResourceUnit, DTMethods)
bindgen wrapper.h --use-core --wrap-unsafe-ops --no-include-path-detection --no-doc-comments --blocklist-item=INCLUDE_VERSION --blocklist-item=LIBRARY_MINIMUM --blocklist-item='.*_t' --blocklist-type=DiskFont --blocklist-type=DiskResourceUnit --blocklist-type=DTMethods > src/cbindings.rs -- -I../NDK3.2R4/Include_H --target=m68k-bare-metal.json

# convert functions to asm
python3 ./scripts/convert-fns.py src/cbindings.rs ../NDK3.2R4/SFD --override=amiga-ndk3.2.0-overrides.json --doc-kickstart-version=true > src/bindings.rs
rm src/cbindings.rs

# add constants missed by rust-bindgen
python3 ./scripts/add-defines.py src/bindings.rs \
    ../NDK3.2R4/Include_H/classes/*.h \
    ../NDK3.2R4/Include_H/datatypes/*.h \
    ../NDK3.2R4/Include_H/devices/*.h \
    ../NDK3.2R4/Include_H/diskfont/*.h \
    ../NDK3.2R4/Include_H/dos/*.h \
    ../NDK3.2R4/Include_H/exec/*.h \
    ../NDK3.2R4/Include_H/gadgets/*.h \
    ../NDK3.2R4/Include_H/graphics/*.h \
    ../NDK3.2R4/Include_H/hardware/*.h \
    ../NDK3.2R4/Include_H/images/*.h \
    ../NDK3.2R4/Include_H/intuition/*.h \
    ../NDK3.2R4/Include_H/libraries/*.h \
    ../NDK3.2R4/Include_H/prefs/*.h \
    ../NDK3.2R4/Include_H/reaction/*.h \
    ../NDK3.2R4/Include_H/resources/*.h \
    ../NDK3.2R4/Include_H/rexx/*.h \
    ../NDK3.2R4/Include_H/utility/*.h \
    ../NDK3.2R4/Include_H/workbench/*.h > src/constants.rs

# fix misnamed structs: these are errors in the original C header files
python3 ./scripts/replace-strings.py '*mut DTMethods' '*mut DTMethod' src/bindings.rs
python3 ./scripts/replace-strings.py '*mut DiskResourceUnit' '*mut DiscResourceUnit' src/bindings.rs
python3 ./scripts/replace-strings.py '*mut DiskFont' '*mut DiskFontHeader' src/bindings.rs

# fix ColorRegister, CIA and RGBTable to have an even size
python3 ./scripts/replace-strings.py 'pub struct ColorRegister {' '#[repr(align(2))] pub struct ColorRegister {' src/bindings.rs
python3 ./scripts/replace-strings.py 'size_of::<ColorRegister>() - 3usize];' 'size_of::<ColorRegister>() - 4usize];' src/bindings.rs
python3 ./scripts/replace-strings.py 'align_of::<ColorRegister>() - 1usize];' 'align_of::<ColorRegister>() - 2usize];' src/bindings.rs

python3 ./scripts/replace-strings.py 'pub struct CIA {' '#[repr(align(2))] pub struct CIA {' src/bindings.rs
python3 ./scripts/replace-strings.py 'size_of::<CIA>() - 3841usize];' 'size_of::<CIA>() - 3842usize];' src/bindings.rs
python3 ./scripts/replace-strings.py 'align_of::<CIA>() - 1usize];' 'align_of::<CIA>() - 2usize];' src/bindings.rs

python3 ./scripts/replace-strings.py 'pub struct RGBTable {' '#[repr(align(2))] pub struct RGBTable {' src/bindings.rs
python3 ./scripts/replace-strings.py 'size_of::<RGBTable>() - 3usize];' 'size_of::<RGBTable>() - 4usize];' src/bindings.rs
python3 ./scripts/replace-strings.py 'align_of::<RGBTable>() - 1usize];' 'align_of::<RGBTable>() - 2usize];' src/bindings.rs

# build it to ensure there's no errors
cargo fmt
cargo build --target m68k-bare-metal.json -Zbuild-std="core" --release
cargo doc
