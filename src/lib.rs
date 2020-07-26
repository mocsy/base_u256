pub mod map_1024;
use map_1024::CHAR1024;

use ux::u30;

// pub trait TriByteEncoder {
//     fn encode(bts: &[u8]) -> (char,char,char);
//     fn decode(chrs: (char,char,char)) -> Vec<u8>;
// }

pub struct BaseU256();
// impl TriByteEncoder for BaseU256 {
impl BaseU256 {
    // Takes 3 bytes at a time and then converts them to 3 utf-8 2 byte characters each with 256 possibilities.
    pub fn encode(bts: &[u8]) -> (char, char, char) {
        let mut iter_expr = bts.iter().take(3).map(|c| CHAR1024[*c as usize]);
        (
            iter_expr.next().unwrap(),
            iter_expr.next().unwrap(),
            iter_expr.next().unwrap(),
        )
    }
    pub fn decode(chrs: (char, char, char)) -> Vec<u8> {
        let (mut a, mut b, mut c) = (0, 0, 0);
        for (idx, ch) in CHAR1024.iter().enumerate() {
            if ch.eq(&chrs.0) {
                a = idx;
            } else if ch.eq(&chrs.1) {
                b = idx;
            } else if ch.eq(&chrs.2) {
                c = idx;
            }
        }
        // O_o
        vec![a as u8, b as u8, c as u8]
    }
}

pub struct BaseU1024();
impl BaseU1024 {
    // Takes 30 bits at a time and then converts them to 3 utf-8 2 byte characters each with 1024 possibilities.
    // Top 2 bits are unused => 30 bits.
    pub fn encode(bts: u30) -> (char, char, char) {
        // // Drop whatever the top 2 bits would have been.
        // let triplet = (bts << 2) >> 2;
        let a = bts >> 20;
        let bc = bts - (a << 20);
        let b = bc >> 10;
        let c = bc - (b << 10);
        let (a, b, c) = (u32::from(a), u32::from(b), u32::from(c));

        assert!(a <= 1024);
        assert!(b <= 1024);
        assert!(c <= 1024);

        let a_char = CHAR1024[a as usize];
        let b_char = CHAR1024[b as usize];
        let c_char = CHAR1024[c as usize];

        (a_char, b_char, c_char)
    }
    pub fn decode(chrs: (char, char, char)) -> u30 {
        let (mut a, mut b, mut c) = (0, 0, 0);
        for (idx, ch) in CHAR1024.iter().enumerate() {
            if ch.eq(&chrs.0) {
                a = idx;
            } else if ch.eq(&chrs.1) {
                b = idx;
            } else if ch.eq(&chrs.2) {
                c = idx;
            }
        }
        // O_o
        u30::new(((a as u32) << 20) + ((b as u32) << 10) + c as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::{BaseU1024, BaseU256};
    use ux::u30;

    #[test]
    fn base_u256_encodes() {
        let f3 = BaseU256::encode(&[0, 1, 2]);
        assert_eq!(('¡', '¢', '£',), f3);

        let mid = BaseU256::encode(&[112, 42, 255]);
        assert_eq!(('Ē', 'Ì', 'ơ',), mid);
    }
    #[test]
    fn base_u256_decodes() {
        let f3 = BaseU256::decode(('¡', '¢', '£'));
        assert_eq!(&[0, 1, 2], f3.as_slice());

        let mid = BaseU256::decode(('Ē', 'Ì', 'ơ'));
        assert_eq!(&[112, 42, 255], mid.as_slice());
    }

    #[test]
    fn base_u1024_encodes() {
        let s1 = BaseU1024::encode(u30::new(1026));
        assert_eq!(('¡', '¢', '£',), s1);

        let s2 = BaseU1024::encode(u30::new((112 << 20) + (42 << 10) + 255));
        assert_eq!(('Ē', 'Ì', 'ơ',), s2);

        let max = BaseU1024::encode(u30::new(1073741823));
        assert_eq!(('֊', '֊', '֊',), max);

        let mid = BaseU1024::encode(u30::new(536870912));
        assert_eq!(('˄', '¡', '¡',), mid);
    }
    #[test]
    fn base_u1024_decodes() {
        let f3 = BaseU1024::decode(('¡', '¢', '£'));
        assert_eq!(u30::new(1026), f3);

        let mid = BaseU1024::decode(('Ē', 'Ì', 'ơ'));
        assert_eq!(u30::new((112 << 20) + (42 << 10) + 255), mid);
    }
}
