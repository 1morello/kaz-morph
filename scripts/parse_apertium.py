#!/usr/bin/env python3
#Parse apertium-kaz lexc file and extract stems with POS tags

import re
import sys
from collections import Counter

# Map continuation classes to our POS
CLASS_TO_POS = {
    # Nouns
    "N1": "Noun", "N5": "Noun", "N6": "Noun",
    "N1-ABBR": "Noun",
    # Verbs
    "V-TV": "Verb", "V-IV": "Verb",
    # Adjectives
    "A1": "Adjective", "A2": "Adjective", "A4": "Adjective",
    # Adverbs
    "ADV": "Adverb",
    # Proper nouns (skip for now)
    # "NP": "ProperNoun",
}

def parse_lexc(path):
    """Extract (lemma, pos) pairs from a .lexc file."""
    entries = []
    seen = set()

    with open(path, encoding="utf-8") as f:
        for line in f:
            line = line.strip()

            # skip comments and empty lines
            if not line or line.startswith("!"):
                continue

            # match pattern: lemma:surface CLASS ;
            # or: lemma CLASS ;
            m = re.match(
                r'^([а-яәғқңөұүһіА-ЯӘҒҚҢӨҰҮҺІ][а-яәғқңөұүһі]*)'
                r'(?::([^\s]+))?\s+'
                r'([A-Z][A-Z0-9\-]*)\s*;',
                line
            )
            if not m:
                continue

            lemma = m.group(1)
            cont_class = m.group(3)

            if cont_class not in CLASS_TO_POS:
                continue

            pos = CLASS_TO_POS[cont_class]
            key = (lemma, pos)

            if key not in seen:
                seen.add(key)
                entries.append((lemma, pos))

    return entries

def main():
    if len(sys.argv) < 2:
        print("Usage: python parse_apertium.py <path-to-lexc>")
        sys.exit(1)

    entries = parse_lexc(sys.argv[1])

    # count by POS
    counts = Counter(pos for _, pos in entries)
    print(f"# Parsed {len(entries)} entries:", file=sys.stderr)
    for pos, count in counts.most_common():
        print(f"#   {pos}: {count}", file=sys.stderr)

    # output as TSV: lemma\tpos
    for lemma, pos in sorted(entries):
        print(f"{lemma}\t{pos}")

if __name__ == "__main__":
    main()
