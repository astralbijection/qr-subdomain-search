extern crate qrcodegen;

use itertools::Itertools;
use qrcodegen::Mask;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use qrcodegen::QrSegment;
use qrcodegen::Version;

fn count_orphans(qr: &QrCode) -> usize {
    let mut orphans = 0;

    let n = qr.size();
    for y in 0..n {
        for x in 0..n {
            let m = qr.get_module(x, y);

            let left = (x != 0) && qr.get_module(x - 1, y);
            let above = (y != 0) && qr.get_module(x, y - 1);
            let right = (x != n - 1) && qr.get_module(x + 1, y);
            let below = (y != n - 1) && qr.get_module(x, y + 1);

            if (above != m) && (below != m) && (right != m) && (left != m) {
                orphans += 1;
            }
        }
    }

    orphans
}

fn min_orphans_for(data: &str) -> (QrCode, usize) {
    let chrs: Vec<char> = data.chars().collect();
    let segs = QrSegment::make_segments(&chrs);

    let qrs: Vec<(QrCode, usize)> = (0..8)
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

            let orphans = count_orphans(&qr);
            (qr, orphans)
        })
        .collect();

    qrs.iter()
        .min_by_key(|(_qr, orphans)| orphans)
        .unwrap()
        .clone()
}

const DNSCHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-";

fn main() {
    for c in 1..5 {
        let subdomains = DNSCHARS
            .chars()
            .permutations(c)
            .map(|cs| cs.into_iter().collect::<String>());

        let domains = subdomains.map(|sd| format!("{}.aay.tw", sd));

        for domain in domains {
            let (qr, orphans) = min_orphans_for(domain.as_str());
            if orphans < 2 {
                println!("{}, {:?}, {}", domain, qr.mask(), orphans);
            }
        }
    }
}
