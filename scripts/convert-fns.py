# converts:
# - functions in bindings.rs to inline assembly
# - function pointers in structs to FPTR

import sys
import re
import json
import os
import argparse

# stores sfd / fd entries
sfd_entries = {}

# override values
overrides = {}

# V number to kickstart version number mapping
# https://wiki.amigaos.net/wiki/AmigaOS_Versions
def v_number_to_ks_version(vstr):
    vnumber = int(vstr[1:])
    vmap = {
        30: "1.0",
        31: "1.1", # NTSC
        32: "1.1", # PAL
        33: "1.2",
        34: "1.3",
        35: "1.3 / A2024 display modes",
        36: "2.0",
        37: "2.04",
        38: "2.1",
        39: "3.0",
        40: "3.1",
        41: "3.1 / Japan",
        42: "3.? / internal alpha",
        43: "3.2 / internal Walker beta",
        44: "3.5 / 1999",
        45: "3.9 / 2000",
        46: "3.1.4 / 2018",
        47: "3.2 / 2020",
        50: "4.0 / AmigaOne Dev PreRelease",
        51: "4.0 / AmigaOne",
        52: "4.0 / PPC",
        53: "4.1 / PPC",
    }
    if vnumber in vmap:
        return vmap[vnumber]
    return "??"

# prints an error message
def eprint(str):
    print("\033[91merror\033[0m: "+str, file=sys.stderr)

def split_last(s, separator):
    lastindex = s.rfind(separator)
    if lastindex == -1:
        return [ s ]
    return [ s[0:lastindex], s[lastindex+len(separator):] ]

def override_value(category, name, field, default_value):
    if category in overrides and name in overrides[category] and field in overrides[category][name]:
        return overrides[category][name][field]
    return default_value

def read_sfd_file(filename):
    lines = []
    short_filename = os.path.basename(filename)
    with open(filename, 'r', encoding='iso-8859-1') as file:
        # combine split lines
        for line_number, line in enumerate(file):
            line = line.replace("\t", "    ")
            if line.startswith("  "):
                lines[-1]["line"] += line.replace("\t", " ")
                continue
            lines.append({ "line": line, "line_number": line_number })

        # process the sfd file line by line
        base = override_value("sfds", short_filename, "base", "")
        basetype = override_value("sfds", short_filename, "basetype", "")
        libname = override_value("sfds", short_filename, "libname", "")
        is_public = True
        offset = 0
        version = override_value("sfds", short_filename, "version", "")
        is_varargs = False
        is_alias = False
        for lineobj in lines:
            line_number = lineobj["line_number"]
            line = lineobj["line"].strip()
            if line == "":
                continue
            elif line.startswith("==bias"):
                offset = -int(line[6:])
                continue
            elif line.startswith("==id"):
                continue
            elif line.startswith("==include"):
                continue
            elif line.startswith("==basetype"):
                basetype = line[10:].strip()
                continue
            elif line.startswith("==base"):
                base = line[8:].strip()
                continue
            elif line.startswith("==libname"):
                libname = line[9:].strip()
                continue
            elif line.startswith("==public"):
                is_public = True
                continue
            elif line.startswith("==private"):
                is_public = False
                continue
            elif line.startswith("==version"):
                version = "V"+line[9:].strip()
                continue
            elif line.startswith("==reserve"):
                reserve = int(line[9:])
                offset -= reserve*6
                continue
            elif line == "==varargs":
                is_varargs = True
                continue
            elif line == "==alias":
                is_alias = True
                continue
            elif line == "==end":
                continue
            elif line.startswith("*"): # skip comments
                continue
            elif line.startswith("=="):
                eprint(f"{filename}:{line_number}: unknown annotation: {line}")
                exit(1)

            # parse function info

            if not line.endswith(")"):
                eprint(f"{filename}:{line_number}: invalid function: {line}")
                exit(1)

            fparts = line.split("(", 1)
            f2parts = split_last(fparts[0].strip(), " ")
            fnname = f2parts[1].strip().replace("*", "")
            if fnname == "":
                eprint(f"{filename}:{line_number}: can't parse sfd function: {line}")
                exit(1)

            rparts = split_last(line, "(")
            registers = rparts[1].strip().replace(")", "").split(",")
            registers = [r.lower().strip() for r in registers]
            if registers[0] == "":
                registers = []

            # vararg and alias functions use the previous function offset
            if is_varargs or is_alias:
                offset += 6
            sfd_entries[fnname] = {
                "base": base,
                "basetype": basetype,
                "libname": libname,
                "name": fnname,
                "registers": registers,
                "public": is_public,
                "offset": offset,
                "version": version,
                "sfd_filename": short_filename,
            }
            offset -= 6
            is_varargs = False
            is_alias = False
    return entries

def read_fd_file(filename):
    short_filename = os.path.basename(filename)
    with open(filename, 'r', encoding='iso-8859-1') as file:
        base = override_value("sfds", short_filename, "base", "")
        basetype = override_value("sfds", short_filename, "basetype", "")
        libname = override_value("sfds", short_filename, "libname", "")
        is_public = True
        offset = 0
        version = override_value("sfds", short_filename, "version", "")
        for line_number, line in enumerate(file):
            line = line.strip()
            if line.startswith("##bias"):
                offset = -int(line[6:])
                continue
            if line.startswith("##base"):
                base = line[8:].strip()
                continue
            elif line.startswith("##public"):
                is_public = True
                continue
            elif line.startswith("##private"):
                is_public = False
                continue
            elif line.startswith("*--- functions in V"): # comment about version
                version = "V" + line[19:21]
                continue
            elif line == "##end":
                continue
            elif line.startswith("*"): # skip comments
                continue

            if line.startswith("##"):
                eprint(f"{filename}:{line_number}: unknown annotation: {line}")
                exit(1)

            # parse function info

            if not line.endswith(")"):
                eprint(f"{filename}:{line_number}: invalid function: {line}")
                exit(1)

            parts = line.split("(")
            if len(parts) != 3:
                eprint(f"{filename}:{line_number}: parts missing in function: {line}")
                exit(1)
            fnname = parts[0].strip()
            fparams = parts[1].replace(")", "").split(",")
            registers = parts[2].replace(")", "").replace("/", ",").split(",")
            registers = [r.lower().strip() for r in registers]
            if registers[0] == "":
                registers = []
            sfd_entries[fnname] = {
                "base": base,
                "basetype": basetype,
                "libname": libname,
                "name": fnname,
                "registers": registers,
                "public": is_public,
                "offset": offset,
                "version": version,
                "sfd_filename": short_filename,
            }
            offset -= 6

def parse_rust_fn(fntext):
    # convert function pointers to FPTR
    fntext = re.sub(r'::core::option::Option<.*? -> .*?>', 'FPTR', fntext)
    fntext = re.sub(r'::core::option::Option<.*?>', 'FPTR', fntext)

    fnt = fntext.replace("pub fn", "").replace(";", "")
    parts = split_last(fnt, "->")
    return_type = "VOID"
    if len(parts) == 2:
        return_type = parts[1].strip()
    parts = parts[0].split("(", 1)
    fnname = parts[0].strip()
    is_vararg = False
    params = []
    for arg in parts[1].replace(")", "").split(","):
        if arg.strip() == "":
            continue
        argparts = arg.strip().split(":", 1)
        argname = argparts[0].strip()
        if argname == "...":
            params.append({
                "name": argname,
                "type": ""
            })
            is_vararg = True
            continue
        argtype = argparts[1].strip()
        params.append({
            "name": argname,
            "type": argtype
        })
    return {
        "name": fnname,
        "params": params,
        "return_type": return_type,
        "is_vararg": is_vararg
    }

def print_asm_implementation(rustfn, filename, line_number):
    # skip vararg functions
    if rustfn["is_vararg"]:
        return

    if not rustfn["name"] in sfd_entries:
        eprint(f"{filename}:{line_number}: sfd/fd entry not found for function: " + rustfn["name"])
        exit(1)

    sfd_entry = sfd_entries[rustfn["name"]]
    basename = sfd_entry["base"]
    libname = sfd_entry["libname"]
    fnoffset = sfd_entry["offset"] # library vector offset (lvo)

    # add register names to fn params
    registers = override_value("functions", rustfn["name"], "registers", sfd_entry["registers"])
    if len(registers) != len(rustfn["params"]):
        eprint(f"{filename}:{line_number}: sfd/fd and C header have different number of params: " + rustfn["name"])
        exit(1)
    for index, sfdreg in enumerate(registers):
        rustfn["params"][index]["register"] = sfdreg

    # add base register a6 as the first parameter
    if basename != "":
        if libname.endswith(".library"):
            # this doesn't use the type given by "basetype", because '*mut Library' doesn't require a cast when called
            basetype = "*mut Library"
        else:
            basetype = "*mut ::core::ffi::c_void"
        rustfn["params"].insert(0, {
            "name": basename,
            "type": basetype,
            "register": "a6"
        })

    # write out the function with inline assembly

    asm_return_type = rustfn["return_type"]
    asm_additional_cast = False
    asm_out = 'out("d0") asm_ret_value,'
    asm_ret_value = "asm_ret_value"
    # u8 or i8 isn't supported by inline asm, so hack around it
    if asm_return_type == "UBYTE":
        asm_return_type = "i16"
        asm_additional_cast = '            "and.w #0x00ff, %d0",' # TODO: is this correct: #0x is a constant?
        asm_ret_value = "asm_ret_value as u8"
    if asm_return_type == "BYTE":
        asm_return_type = "i16"
        asm_additional_cast = '            "ext.w %d0",'
        asm_ret_value = "asm_ret_value as i8"
    # f32 or f64 are not supported by inline asm, so convert them to u32 or (u32,u32)
    if asm_return_type == "FLOAT":
        asm_return_type = "u32"
        asm_ret_value = "f32::from_bits(asm_ret_value)"
    elif asm_return_type == "DOUBLE":
        asm_return_type = "(u32, u32) = (0, 0)"
        asm_out = 'out("d0") asm_ret_value.0,\n' + '            out("d1") asm_ret_value.1,'
        asm_ret_value = "f64::from_bits(((asm_ret_value.0 as u64) << 32) | asm_ret_value.1 as u64)"

    rust_params = []
    asm_in = []
    saved_regs = []
    pre_move_instr = []
    for param in rustfn["params"]:
        param_name = param["name"]
        reg = param["register"]
        if param_name == "type": # reserved word in Rust
            param_name = "type_"
        rust_params.append(f'{param_name}: {param["type"]}')

        if reg == "a4" or reg == "a5":
            # a4 and a5 must be passed in using a temp reg because LLVM uses them
            saved_regs.append(reg)
            if param["type"] == "FPTR":
                asm_in.append(f'            {reg}reg = in(reg) {param_name},')
                # use if FPTR is changed to be '::core::option::Option<unsafe extern "system" fn()>'
                #asm_in.append(f'            {reg}reg = in(reg) match {param_name} {{ Some(f) => f as usize, None => 0 }},')
            else:
                asm_in.append(f'            {reg}reg = in(reg) {param_name},')
            pre_move_instr.append(f'            "move.l {{{reg}reg}}, %{reg}",')

        elif reg == "a6":
            if not ("Base" in param_name or "Device" in param_name or "Resource" in param_name or param_name == "resource"):
                eprint(f"{filename}:{line_number}: a6 not used for lib base addr: {line}")
                exit(1)
            # a6 must be passed in using a temp reg because LLVM uses it
            saved_regs.append(reg)
            asm_in.append(f'            basereg = in(reg) {param_name},')
            pre_move_instr.append(f'            "move.l {{basereg}}, %a6",')

        else:
            if param["type"] == "FLOAT":
                asm_in.append(f'            in("{reg}") {param_name}.to_bits(),')
            elif param["type"] == "DOUBLE":
                asm_in.append(f'            in("{reg.split("-")[0]}") ({param_name}.to_bits() >> 32) as u32,')
                asm_in.append(f'            in("{reg.split("-")[1]}") ({param_name}.to_bits() & 0xffff_ffff) as u32,')
            elif param["type"] == "FPTR":
                asm_in.append(f'            in("{reg}") {param_name},')
                # use if FPTR is changed to be '::core::option::Option<unsafe extern "system" fn()>'
                #asm_in.append(f'            in("{reg}") match {param_name} {{ Some(f) => f as usize, None => 0 }},')
            else:
                asm_in.append(f'            in("{reg}") {param_name},')

        if reg == "d0" or reg == "d0-d1":
            asm_out = asm_out.replace("out", "lateout")

    # doc comment for function

    doc_comment = f'libname: "{libname}"'

    minversion = sfd_entry["version"]
    minversion = override_value("sfds", sfd_entry["sfd_filename"], "version", minversion)
    minversion = override_value("functions", rustfn["name"], "version", minversion)
    if minversion != "":
        doc_comment += f' ({minversion})'
        if args.doc_ks_version_bool:
            doc_comment += f' (Kickstart {v_number_to_ks_version(minversion)})'

    maxversion = override_value("sfds", sfd_entry["sfd_filename"], "maxversion", "")
    maxversion = override_value("functions", rustfn["name"], "maxversion", maxversion)
    if maxversion != "":
        doc_comment += f' (maximum {maxversion})'
        if args.doc_ks_version_bool:
            doc_comment += f' (Kickstart {v_number_to_ks_version(maxversion)})'

    if args.doc_comments_bool:
        print("/// " + doc_comment)

    # function signature
    if rustfn["return_type"] == "VOID":
        print(f'pub unsafe fn {rustfn["name"]}({", ".join(rust_params)}) {{')
    else:
        print(f'pub unsafe fn {rustfn["name"]}({", ".join(rust_params)}) -> {rustfn["return_type"]} {{')
        if asm_return_type == "(u32, u32) = (0, 0)":
            print(f'    let mut asm_ret_value: {asm_return_type};')
        else:
            print(f'    let asm_ret_value: {asm_return_type};')

    # inline assembly uses .short values because Rust inline m68k doesn't support some instructions
    print(f'    unsafe {{')
    print(f'        asm!(')
    # save d0-d1 and a0-a1 because system functions treat them as scratch registers
    if rustfn["return_type"] == "VOID":
        print(f'            ".short 0x48e7", // movem.l %d0-%d1/%a0-%a1, -(%sp)')
        print(f'            ".short 0xc0c0",')
    else:
        print(f'            ".short 0x48e7", // movem.l %d1/%a0-%a1, -(%sp)')
        print(f'            ".short 0x40c0",')
    for sreg in saved_regs:
        print(f'            "move.l %{sreg}, -(%sp)",')
    for pmi in pre_move_instr:
        print(pmi)
    print(f'            ".short 0x4eae", // jsr {fnoffset}(a6)')
    print(f'            ".short {fnoffset}",')
    if asm_additional_cast:
        print(asm_additional_cast)
    for sreg in reversed(saved_regs):
        print(f'            "move.l (%sp)+, %{sreg}",')
    if rustfn["return_type"] == "VOID":
        print(f'            "movem.l (%sp)+, %d0-%d1/%a0-%a1",')
    else:
        print(f'            "movem.l (%sp)+, %d1/%a0-%a1",')
    for ain in asm_in:
        print(ain)
    if rustfn["return_type"] != "VOID":
        print(f'            {asm_out}')
    print(f'        );')
    print(f'    }}')
    if rustfn["return_type"] != "VOID":
        print(f'    {asm_ret_value}')
    print(f'}}')
    print(f'')

## main

parser = argparse.ArgumentParser(description="Convert bindings to use Amiga library calling convention.")
parser.add_argument('bindings_rs', nargs=1,
                    help='path to bindings.rs created by rust-bindgen')
parser.add_argument('sfd_folder', nargs=1,
                    help='path to the sfd or fd folder')
parser.add_argument('--doc-comments', dest='doc_comments_bool',
                    type=lambda x: (str(x).lower() == 'true'), default=True,
                    help='add doc comments [default: true]')
parser.add_argument('--doc-kickstart-version', dest='doc_ks_version_bool',
                    type=lambda x: (str(x).lower() == 'true'), default=False,
                    help='add Kickstart version to doc comments [default: false]')
parser.add_argument('--override', nargs=1, type=str, dest='overridejson',
                    help='filename of the override file')
args = parser.parse_args()

# read overrides.json file: it overrides values defined in sfd files
if args.overridejson:
    with open(args.overridejson[0], 'r') as file:
        overrides = json.load(file)

# read sfd / fd files
dirpath = args.sfd_folder[0]
entries = os.listdir(dirpath)
for entry in entries:
    filename = os.path.join(dirpath, entry)
    if filename.endswith(".sfd"):
        read_sfd_file(filename)
    elif filename.endswith(".fd"):
        read_fd_file(filename)

# collect bindgen generated bindings to an array of lines
bfilename = args.bindings_rs[0]
blines = []
with open(bfilename, 'r') as file:
    for line_number, line in enumerate(file):
        blines.append({ "line": line, "line_number": line_number })

# print some additional definitions
print('use core::arch::asm;')
# TODO: is this correct way to define function pointers that match Amiga function pointers (FPTR)?
print('\n/// A function pointer with the Amiga calling convention')
print('///')
print('/// Parameters and the return value are passed in the CPU registers (this is not the "C"')
print('/// calling convention). The ROM kernel manuals have documented which registers to use.')
print('pub type FPTR = usize;\n')
#future?: print('type FPTR = ::core::option::Option<unsafe extern "system" fn()>;\n')
print('/* automatically modified by the amiga-sys tools */')

# process bindings line by line
inside_fn = False
inside_struct = False
inside_struct_option = False
rustfntext = ""
for lineobj in blines:
    line = lineobj["line"]
    line_number = lineobj["line_number"]
    line = line.replace("\n", "")

    # process functions: parse them, modify them (FPTRs) and print the asm code
    if inside_fn == True:
        if line == "}":
            inside_fn = False
            rustfn = parse_rust_fn(rustfntext.strip())
            print_asm_implementation(rustfn, bfilename, line_number)
            continue
        rustfntext += " " + line.strip()
        continue
    if line.startswith('unsafe extern "C" {'):
        inside_fn = True
        rustfntext = ""
        continue

    # process structs: convert fields with function pointers to FPTR
    if inside_struct == True:
        # process a multiline Option<> inside a struct
        if inside_struct_option == True:
            if line.startswith("    >"):
                line = line.replace("    >", "")
                inside_struct_option = False
            else:
                line = ""
        # start of Option<>, which is the start of a function pointer
        if line.endswith("::core::option::Option<"):
            line = line.replace("::core::option::Option<", "FPTR")
            inside_struct_option = True
        if '::core::option::Option<unsafe extern "C" fn(' in line:
            index = line.find('::core::option::Option<unsafe extern "C" fn(')
            if index >= 0:
                line = line[:index] + "FPTR,"
        # end of struct
        if line == "}":
            inside_struct = False
    # start of struct
    if line.startswith('pub struct '):
        inside_struct = True

    # process type definitions: convert function pointers to FPTR
    if line.startswith("pub type "):
        if '::core::option::Option<unsafe extern "C" fn(' in line:
            index = line.find('::core::option::Option<unsafe extern "C" fn(')
            if index >= 0:
                line = line[:index] + "FPTR;"

    # these are ugly and could be marked as errors, but let's leave them as they are
    if line == "    pub _address: u8,":
        pass
        #eprint(f"bindings.rs:{line_number}: rust-bindgen generated filler field: {line}")
        #exit(1)
    if " _unused" in line:
        pass
        #eprint(f"bindings.rs:{line_number}: rust-bindgen generated filler field: {line}")
        #exit(1)
    if "__BindgenBitfieldUnit" in line:
        pass
        #eprint(f"bindings.rs:{line_number}: rust-bindgen generated __BindgenBitfieldUnit: {line}")
        #exit(1)
    if "__IncompleteArrayField" in line:
        pass
        #eprint(f"bindings.rs:{line_number}: rust-bindgen generated __IncompleteArrayField: {line}")
        #exit(1)
    print(line)
