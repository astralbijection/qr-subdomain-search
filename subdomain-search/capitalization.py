#!/usr/bin/env python3

from qrcodegen import *


def get_urls():
    for i in range(2**5):
        url = list('s3e.top')
        if i & 1:
            url[0] = 'S'
        if i & 2:
            url[2] = 'E'
        if i & 4:
            url[4] = 'T'
        if i & 8:
            url[5] = 'O'
        if i & 16:
            url[6] = 'P'
        yield ''.join(url)

def count_full_cells(qr: QrCode):
    return sum((
        1 if qr.get_module(x, y) else 0
        for y in range(qr.get_size())
        for x in range(qr.get_size())
    ))


def to_svg_str(qr: QrCode, border: int) -> str:
	"""Returns a string of SVG code for an image depicting the given QR Code, with the given number
	of border modules. The string always uses Unix newlines (\n), regardless of the platform.

    Yoinked from https://github.com/nayuki/QR-Code-generator/blob/master/python/qrcodegen-demo.py"""
	if border < 0:
		raise ValueError("Border must be non-negative")
	parts: List[str] = []
	for y in range(qr.get_size()):
		for x in range(qr.get_size()):
			if qr.get_module(x, y):
				parts.append("M{},{}h1v1h-1z".format(x + border, y + border))
	return """<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {0} {0}" stroke="none">
	<rect width="100%" height="100%" fill="#FFFFFF"/>
	<path d="{1}" fill="#000000"/>
</svg>
""".format(qr.get_size() + border * 2, " ".join(parts))


def get_qrs():
    for url in get_urls():
        segs = QrSegment.make_segments(url)
        for mask in range(0, 8):
            qr = QrCode.encode_segments(segs, QrCode.Ecc.HIGH, 1, 1, mask, False)
            yield url, mask, qr


qrs = list(get_qrs())
qrs.sort(key=lambda x: count_full_cells(x[2]))
for u, m, q in qrs[:100]:
    print(f'out/{u}_{m}.svg')

