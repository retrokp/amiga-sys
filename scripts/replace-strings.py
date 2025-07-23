# replaces all occurances of from-string with to-string in the given file

import sys

if len(sys.argv) != 4:
    print(f"usage: {sys.argv[0]} from-string to-string filename")
    exit(1)

from_string = sys.argv[1].replace("\\n", "\n")
to_string = sys.argv[2].replace("\\n", "\n")

with open(sys.argv[3]) as f:
    new_text = f.read().replace(from_string, to_string)

with open(sys.argv[3], "w") as f:
    f.write(new_text)
