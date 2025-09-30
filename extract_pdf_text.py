import sys
import re
import zlib
from pathlib import Path

sys.stdout.reconfigure(encoding='utf-8')

WHITESPACE = set(b" \t\r\n\x0c\x00")


def read_string(data: bytes, i: int):
    buf = bytearray()
    depth = 1
    while i < len(data) and depth > 0:
        c = data[i]
        if c == 0x5c:  # backslash
            i += 1
            if i >= len(data):
                break
            esc = data[i]
            if esc in b"nrtbf()\\":
                mapping = {
                    ord(b"n"): "\n",
                    ord(b"r"): "\r",
                    ord(b"t"): "\t",
                    ord(b"b"): "\b",
                    ord(b"f"): "\f",
                    ord(b"("): "(",
                    ord(b")"): ")",
                    ord(b"\\"): "\\",
                }
                buf.extend(mapping.get(esc, chr(esc)).encode('latin-1'))
            elif ord('0') <= esc <= ord('7'):
                oct_digits = [esc]
                j = 1
                while j < 3 and i + j < len(data):
                    nxt = data[i + j]
                    if ord('0') <= nxt <= ord('7'):
                        oct_digits.append(nxt)
                        j += 1
                    else:
                        break
                i += len(oct_digits) - 1
                buf.append(int(bytes(oct_digits), 8))
            else:
                buf.append(esc)
        elif c == 0x28:
            depth += 1
            buf.append(c)
        elif c == 0x29:
            depth -= 1
            if depth == 0:
                i += 1
                break
            buf.append(c)
        else:
            buf.append(c)
        i += 1
    return buf.decode('latin-1', errors='ignore'), i


def skip_comment(data: bytes, i: int):
    while i < len(data) and data[i] not in (10, 13):
        i += 1
    return i


def skip_dictionary(data: bytes, i: int):
    depth = 1
    i += 2  # skip initial '<<'
    while i < len(data) and depth > 0:
        if data[i] == 0x25:  # '%'
            i = skip_comment(data, i + 1)
        elif data[i:i + 2] == b'<<':
            depth += 1
            i += 2
        elif data[i:i + 2] == b'>>':
            depth -= 1
            i += 2
        else:
            i += 1
    return i


def read_hex_string(data: bytes, i: int):
    i += 1  # skip '<'
    hex_chars = []
    while i < len(data) and data[i] != 0x3e:  # '>'
        c = data[i]
        if c not in WHITESPACE:
            hex_chars.append(chr(c))
        i += 1
    if i < len(data) and data[i] == 0x3e:
        i += 1
    hex_str = ''.join(hex_chars)
    if len(hex_str) % 2 == 1:
        hex_str += '0'
    try:
        text = bytes.fromhex(hex_str).decode('latin-1', errors='ignore')
    except ValueError:
        text = ''
    return text, i


def read_array(data: bytes, i: int):
    arr = []
    i += 1  # skip '['
    while i < len(data):
        c = data[i]
        if c in WHITESPACE:
            i += 1
            continue
        if c == 0x5d:  # ']'
            return arr, i + 1
        if c == 0x25:  # '%'
            i = skip_comment(data, i + 1)
            continue
        if c == 0x28:
            text, i = read_string(data, i + 1)
            arr.append(text)
            continue
        if c == 0x5b:  # nested array
            sub, i = read_array(data, i)
            arr.append(sub)
            continue
        if c == 0x3c:  # '<'
            if i + 1 < len(data) and data[i + 1] == 0x3c:
                i = skip_dictionary(data, i)
                continue
            text, i = read_hex_string(data, i)
            arr.append(text)
            continue
        if c == 0x2f:  # name, ignore
            i += 1
            while i < len(data) and data[i] not in WHITESPACE and data[i] not in b'[]()<>{}':
                i += 1
            continue
        j = i
        while j < len(data) and data[j] not in WHITESPACE and data[j] not in b'[]()<>{}/':
            j += 1
        token = data[i:j].decode('latin-1', errors='ignore')
        if token:
            try:
                num = float(token)
                arr.append(int(num) if num.is_integer() else num)
            except ValueError:
                arr.append(token)
        i = j
    return arr, i


def tokenize(data: bytes):
    i = 0
    n = len(data)
    while i < n:
        c = data[i]
        if c in WHITESPACE:
            i += 1
            continue
        if c == 0x25:  # '%'
            i = skip_comment(data, i + 1)
            continue
        if c == 0x28:
            text, i = read_string(data, i + 1)
            yield ('STRING', text)
            continue
        if c == 0x3c:  # '<'
            if i + 1 < n and data[i + 1] == 0x3c:
                i = skip_dictionary(data, i)
                continue
            text, i = read_hex_string(data, i)
            yield ('STRING', text)
            continue
        if c == 0x5b:
            arr, i = read_array(data, i)
            yield ('ARRAY', arr)
            continue
        if c == 0x2f:  # '/' name
            i += 1
            start = i
            while i < n and data[i] not in WHITESPACE and data[i] not in b'[]()<>{}/':
                i += 1
            name = data[start:i].decode('latin-1', errors='ignore')
            yield ('NAME', '/' + name)
            continue
        j = i
        while j < n and data[j] not in WHITESPACE and data[j] not in b'[]()<>{}/':
            j += 1
        token = data[i:j].decode('latin-1', errors='ignore')
        if token:
            try:
                num = float(token)
                yield ('NUMBER', int(num) if num.is_integer() else num)
            except ValueError:
                yield ('NAME', token)
        i = j


def interpret(tokens):
    stack = []
    lines = []
    current = ''

    def flush(force_blank=False):
        nonlocal current
        if current:
            lines.append(current)
            current = ''
        elif force_blank:
            lines.append('')

    for typ, val in tokens:
        if typ in ('STRING', 'ARRAY', 'NUMBER'):
            stack.append((typ, val))
            continue
        if typ != 'NAME':
            continue
        op = val
        if op == 'Tj':
            if stack and stack[-1][0] == 'STRING':
                current += stack.pop()[1]
            stack.clear()
        elif op == "'":
            if stack and stack[-1][0] == 'STRING':
                flush()
                current += stack.pop()[1]
            stack.clear()
        elif op == '"':
            if len(stack) >= 3 and stack[-1][0] == 'STRING':
                text = stack.pop()[1]
                stack.pop()
                stack.pop()
                flush()
                current += text
            stack.clear()
        elif op == 'TJ':
            if stack and stack[-1][0] == 'ARRAY':
                arr = stack.pop()[1]
                for item in arr:
                    if isinstance(item, str):
                        current += item
                    elif isinstance(item, (int, float)):
                        if item < -120:
                            current += ' '
                stack.clear()
        elif op in ('Td', 'TD', 'Tm'):
            flush()
            stack.clear()
        elif op == 'ET':
            flush()
            stack.clear()
        elif op == 'BT':
            flush()
            stack.clear()
        else:
            stack.clear()
    flush()
    return lines


def extract_text(path: Path):
    data = path.read_bytes()
    pattern = re.compile(rb'stream\r?\n(.*?)\r?\nendstream', re.DOTALL)
    lines = []
    for raw in pattern.findall(data):
        chunk = raw
        try:
            chunk = zlib.decompress(raw)
        except Exception:
            pass
        tokens = list(tokenize(chunk))
        if not tokens:
            continue
        lines.extend(interpret(tokens))
    cleaned = []
    prev_blank = False
    for line in lines:
        text = line.strip()
        if not text:
            if not prev_blank:
                cleaned.append('')
            prev_blank = True
        else:
            cleaned.append(text)
            prev_blank = False
    return '\n'.join(cleaned)


def main(path_str: str):
    path = Path(path_str)
    text = extract_text(path)
    print(text)


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('usage: extract_pdf_text.py <pdf_path>', file=sys.stderr)
        sys.exit(1)
    main(sys.argv[1])
