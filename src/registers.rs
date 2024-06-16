#[derive(Debug, Clone, Copy)]
enum FlagKind {
    Z,
    N,
    H,
    C,
}

#[derive(Debug, Clone, Copy)]
enum R8Kind {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Clone, Copy)]
enum R16Kind {
    AF,
    BC,
    DE,
    HL,
    SP,
}

const MOST_SIGNIFICANT_BYTE: usize = 1;
const LEAST_SIGNIFICANT_BYTE: usize = 0;

#[derive(Debug)]
struct Registers {
    /// AF Register, high byte A, low byte F
    /// Layout of flags in F register: ZHNC----
    af: [u8; 2],
    /// BC Register, high byte B, low byte C
    bc: [u8; 2],
    /// DE Register, high byte D, low byte E
    de: [u8; 2],
    /// HL Register, high byte H, low byte L
    hl: [u8; 2],
    /// stack pointer
    sp: u16,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            af: [0, 0],
            bc: [0, 0],
            de: [0, 0],
            hl: [0, 0],
            sp: 0,
        }
    }

    pub fn get_flag(&self, flag_kind: FlagKind) -> bool {
        let flag_register = self.af[LEAST_SIGNIFICANT_BYTE];
        let flag_value = match flag_kind {
            FlagKind::Z => (flag_register >> 7) & 1,
            FlagKind::N => (flag_register >> 6) & 1,
            FlagKind::H => (flag_register >> 5) & 1,
            FlagKind::C => (flag_register >> 4) & 1,
        };
        flag_value != 0
    }

    pub fn set_flag(&mut self, flag_kind: FlagKind, value: bool) {
        let mask = match flag_kind {
            FlagKind::Z => 0b1000_0000,
            FlagKind::N => 0b0100_0000,
            FlagKind::H => 0b0010_0000,
            FlagKind::C => 0b0001_0000,
        };
        let flag_register = &mut self.af[LEAST_SIGNIFICANT_BYTE];
        if value {
            *flag_register |= mask;
        } else {
            *flag_register &= !mask;
        }
    }

    pub fn get_r8(&self, r8kind: R8Kind) -> u8 {
        match r8kind {
            R8Kind::A => self.af[MOST_SIGNIFICANT_BYTE],
            R8Kind::B => self.bc[MOST_SIGNIFICANT_BYTE],
            R8Kind::C => self.bc[LEAST_SIGNIFICANT_BYTE],
            R8Kind::D => self.de[MOST_SIGNIFICANT_BYTE],
            R8Kind::E => self.de[LEAST_SIGNIFICANT_BYTE],
            R8Kind::H => self.hl[MOST_SIGNIFICANT_BYTE],
            R8Kind::L => self.hl[LEAST_SIGNIFICANT_BYTE],
        }
    }

    pub fn get_mut_r8(&mut self, r8kind: R8Kind) -> &mut u8 {
        match r8kind {
            R8Kind::A => &mut self.af[MOST_SIGNIFICANT_BYTE],
            R8Kind::B => &mut self.bc[MOST_SIGNIFICANT_BYTE],
            R8Kind::C => &mut self.bc[LEAST_SIGNIFICANT_BYTE],
            R8Kind::D => &mut self.de[MOST_SIGNIFICANT_BYTE],
            R8Kind::E => &mut self.de[LEAST_SIGNIFICANT_BYTE],
            R8Kind::H => &mut self.hl[MOST_SIGNIFICANT_BYTE],
            R8Kind::L => &mut self.hl[LEAST_SIGNIFICANT_BYTE],
        }
    }

    pub fn get_r16(&self, r16kind: R16Kind) -> u16 {
        let double_register = match r16kind {
            R16Kind::AF => &self.af,
            R16Kind::BC => &self.bc,
            R16Kind::DE => &self.de,
            R16Kind::HL => &self.hl,
            R16Kind::SP => return self.sp,
        };
        // SAFETY: The cast from `[u8; 2]` to `u16` is safe since both types have the same layout.
        // The pointer dereference is safe since it comes from a reference which is guaranteed to
        // be valid for reads.
        let pointer = double_register.as_ptr() as *const u16;
        unsafe { *pointer }
    }

    pub fn get_mut_r16(&mut self, r16kind: R16Kind) -> &mut u16 {
        let double_register = match r16kind {
            R16Kind::AF => &mut self.af,
            R16Kind::BC => &mut self.bc,
            R16Kind::DE => &mut self.de,
            R16Kind::HL => &mut self.hl,
            R16Kind::SP => return &mut self.sp,
        };
        // SAFETY: The cast from `[u8; 2]` to `u16` is safe since both types have the same layout.
        // The pointer dereference is safe since it comes from a mutable reference which is
        // guaranteed to be valid for writes.
        let pointer = double_register.as_ptr() as *mut u16;
        unsafe { &mut *pointer }
    }
}

#[cfg(test)]
mod test {
    use crate::registers::{FlagKind, R16Kind, R8Kind, Registers};

    #[test]
    fn r16_access() {
        let mut r = Registers::new();
        assert_eq!(r.get_r16(R16Kind::AF), 0);
        assert_eq!(r.get_r16(R16Kind::BC), 0);
        assert_eq!(r.get_r16(R16Kind::DE), 0);
        assert_eq!(r.get_r16(R16Kind::HL), 0);
        assert_eq!(r.get_r16(R16Kind::SP), 0);
        *r.get_mut_r16(R16Kind::AF) = 300;
        *r.get_mut_r16(R16Kind::BC) = 400;
        *r.get_mut_r16(R16Kind::DE) = 500;
        *r.get_mut_r16(R16Kind::HL) = 600;
        *r.get_mut_r16(R16Kind::SP) = 700;
        assert_eq!(r.get_r16(R16Kind::AF), 300);
        assert_eq!(r.get_r16(R16Kind::BC), 400);
        assert_eq!(r.get_r16(R16Kind::DE), 500);
        assert_eq!(r.get_r16(R16Kind::HL), 600);
        assert_eq!(r.get_r16(R16Kind::SP), 700);
    }

    #[test]
    fn mixed_access_af() {
        let mut r = Registers::new();
        *r.get_mut_r16(R16Kind::AF) = 0b0000_0001_1010_0000;
        assert_eq!(r.get_r16(R16Kind::AF), 0b0000_0001_1010_0000);
        assert_eq!(r.get_r8(R8Kind::A), 1);
        assert!(r.get_flag(FlagKind::Z));
        assert!(!r.get_flag(FlagKind::N));
        assert!(r.get_flag(FlagKind::H));
        assert!(!r.get_flag(FlagKind::C));
    }

    #[test]
    fn mixed_access_bc() {
        let mut r = Registers::new();
        *r.get_mut_r8(R8Kind::B) = 1;
        *r.get_mut_r8(R8Kind::C) = 2;
        assert_eq!(r.get_r16(R16Kind::BC), 258);
        assert_eq!(r.get_r8(R8Kind::B), 1);
        assert_eq!(r.get_r8(R8Kind::C), 2);
    }

    #[test]
    fn mixed_access_de() {
        let mut r = Registers::new();
        *r.get_mut_r8(R8Kind::D) = 1;
        *r.get_mut_r8(R8Kind::E) = 2;
        assert_eq!(r.get_r16(R16Kind::DE), 258);
        assert_eq!(r.get_r8(R8Kind::D), 1);
        assert_eq!(r.get_r8(R8Kind::E), 2);
    }

    #[test]
    fn mixed_access_hl() {
        let mut r = Registers::new();
        *r.get_mut_r16(R16Kind::HL) = 258;
        assert_eq!(r.get_r16(R16Kind::HL), 258);
        assert_eq!(r.get_r8(R8Kind::H), 1);
        assert_eq!(r.get_r8(R8Kind::L), 2);
    }
}
