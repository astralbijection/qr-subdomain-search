extern crate qrcodegen;

use itertools::Itertools;
use qrcodegen::Mask;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use qrcodegen::QrSegment;
use qrcodegen::Version;

fn count_orphans(qr: &QrCode) -> usize {
    let mut orphans = 0;

    for y in 1..qr.size() - 1 {
        for x in 1..qr.size() - 1 {
            let m = qr.get_module(x, y);

            if qr.get_module(x - 1, y) != m
                && qr.get_module(x, y - 1) != m
                && qr.get_module(x + 1, y) != m
                && qr.get_module(x, y + 1) != m
            {
                orphans += 1;
            }
        }
    }

    orphans
}

fn min_orphans_for(data: &str) -> QrCode {
    let chrs: Vec<char> = data.chars().collect();
    let segs = QrSegment::make_segments(&chrs);

    (0..8)
        .map(|mask| {
            let qr = QrCode::encode_segments_advanced(
                &segs,
                QrCodeEcc::Quartile,
                Version::new(1),
                Version::new(1),
                Some(Mask::new(mask)),
                false,
            )
            .unwrap();
            qr
        })
        .min_by_key(|qr| count_orphans(&qr))
        .unwrap()
}

const DNSCHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-";

fn main() {
    let subdomains = DNSCHARS
        .chars()
        .combinations(3)
        .map(|cs| cs.into_iter().collect::<String>());

    let domains = subdomains.map(|sd| format!("{}.aay.tw", sd));

    for domain in domains {
        let qr = min_orphans_for(domain.as_str());
        let orphans = count_orphans(&qr);
        if orphans < 4 {
            println!("{}, {:?}, {}", domain, qr.mask(), orphans);
        }
    }
}
