extern crate qrcodegen;

use itertools::Itertools;
use qrcodegen::Mask;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use qrcodegen::QrSegment;
use qrcodegen::Version;
use rayon::prelude::*;

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

const DNSCHARS: [char; 37] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-',
];

fn generate_domain(base: &str, max_len: usize) -> String {
    use rand::prelude::*;
    let mut rng = thread_rng();

    let append_len = rng.gen_range(1..(max_len - base.len()));
    let mut subdomain_len = rng.gen_range(0..append_len);

    // Host cannot start with '.' so .foo.com is out
    if subdomain_len > 0 {
        subdomain_len += 1;
    }
    let path_len = append_len - subdomain_len;

    let mut chars = Vec::new();

    if subdomain_len > 0 {
        chars.extend(DNSCHARS.choose_multiple(&mut rng, subdomain_len));
        chars.push('.');
    }

    chars.extend(base.chars());

    if path_len > 0 {
        chars.push('/');
        chars.extend(DNSCHARS.choose_multiple(&mut rng, path_len));
    }

    for i in 0..chars.len() {
        if rng.gen_ratio(1, 2) {
            chars[i] = chars[i].to_uppercase().collect::<Vec<_>>()[0];
        }
    }

    chars.into_iter().collect()
}

fn main() {
    let is: Vec<usize> = (0..100000000).collect();
    is.par_iter()
        .map(|_| generate_domain("s3e.top", 10))
        .map(|domain| {
            let (qr, orphans) = min_orphans_for(domain.as_str());
            (domain, qr.mask(), orphans)
        })
        .for_each(|(domain, qm, o)| {
            if o <= 1 {
                println!("{}\t{}\t{}", domain, qm.value(), o);
            }
        });
}
