use std::fmt;
use libc;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ud_type {
    UD_NONE = 0,
    UD_R_AL = 1,
    UD_R_CL = 2,
    UD_R_DL = 3,
    UD_R_BL = 4,
    UD_R_AH = 5,
    UD_R_CH = 6,
    UD_R_DH = 7,
    UD_R_BH = 8,
    UD_R_SPL = 9,
    UD_R_BPL = 10,
    UD_R_SIL = 11,
    UD_R_DIL = 12,
    UD_R_R8B = 13,
    UD_R_R9B = 14,
    UD_R_R10B = 15,
    UD_R_R11B = 16,
    UD_R_R12B = 17,
    UD_R_R13B = 18,
    UD_R_R14B = 19,
    UD_R_R15B = 20,
    UD_R_AX = 21,
    UD_R_CX = 22,
    UD_R_DX = 23,
    UD_R_BX = 24,
    UD_R_SP = 25,
    UD_R_BP = 26,
    UD_R_SI = 27,
    UD_R_DI = 28,
    UD_R_R8W = 29,
    UD_R_R9W = 30,
    UD_R_R10W = 31,
    UD_R_R11W = 32,
    UD_R_R12W = 33,
    UD_R_R13W = 34,
    UD_R_R14W = 35,
    UD_R_R15W = 36,
    UD_R_EAX = 37,
    UD_R_ECX = 38,
    UD_R_EDX = 39,
    UD_R_EBX = 40,
    UD_R_ESP = 41,
    UD_R_EBP = 42,
    UD_R_ESI = 43,
    UD_R_EDI = 44,
    UD_R_R8D = 45,
    UD_R_R9D = 46,
    UD_R_R10D = 47,
    UD_R_R11D = 48,
    UD_R_R12D = 49,
    UD_R_R13D = 50,
    UD_R_R14D = 51,
    UD_R_R15D = 52,
    UD_R_RAX = 53,
    UD_R_RCX = 54,
    UD_R_RDX = 55,
    UD_R_RBX = 56,
    UD_R_RSP = 57,
    UD_R_RBP = 58,
    UD_R_RSI = 59,
    UD_R_RDI = 60,
    UD_R_R8 = 61,
    UD_R_R9 = 62,
    UD_R_R10 = 63,
    UD_R_R11 = 64,
    UD_R_R12 = 65,
    UD_R_R13 = 66,
    UD_R_R14 = 67,
    UD_R_R15 = 68,
    UD_R_ES = 69,
    UD_R_CS = 70,
    UD_R_SS = 71,
    UD_R_DS = 72,
    UD_R_FS = 73,
    UD_R_GS = 74,
    UD_R_CR0 = 75,
    UD_R_CR1 = 76,
    UD_R_CR2 = 77,
    UD_R_CR3 = 78,
    UD_R_CR4 = 79,
    UD_R_CR5 = 80,
    UD_R_CR6 = 81,
    UD_R_CR7 = 82,
    UD_R_CR8 = 83,
    UD_R_CR9 = 84,
    UD_R_CR10 = 85,
    UD_R_CR11 = 86,
    UD_R_CR12 = 87,
    UD_R_CR13 = 88,
    UD_R_CR14 = 89,
    UD_R_CR15 = 90,
    UD_R_DR0 = 91,
    UD_R_DR1 = 92,
    UD_R_DR2 = 93,
    UD_R_DR3 = 94,
    UD_R_DR4 = 95,
    UD_R_DR5 = 96,
    UD_R_DR6 = 97,
    UD_R_DR7 = 98,
    UD_R_DR8 = 99,
    UD_R_DR9 = 100,
    UD_R_DR10 = 101,
    UD_R_DR11 = 102,
    UD_R_DR12 = 103,
    UD_R_DR13 = 104,
    UD_R_DR14 = 105,
    UD_R_DR15 = 106,
    UD_R_MM0 = 107,
    UD_R_MM1 = 108,
    UD_R_MM2 = 109,
    UD_R_MM3 = 110,
    UD_R_MM4 = 111,
    UD_R_MM5 = 112,
    UD_R_MM6 = 113,
    UD_R_MM7 = 114,
    UD_R_ST0 = 115,
    UD_R_ST1 = 116,
    UD_R_ST2 = 117,
    UD_R_ST3 = 118,
    UD_R_ST4 = 119,
    UD_R_ST5 = 120,
    UD_R_ST6 = 121,
    UD_R_ST7 = 122,
    UD_R_XMM0 = 123,
    UD_R_XMM1 = 124,
    UD_R_XMM2 = 125,
    UD_R_XMM3 = 126,
    UD_R_XMM4 = 127,
    UD_R_XMM5 = 128,
    UD_R_XMM6 = 129,
    UD_R_XMM7 = 130,
    UD_R_XMM8 = 131,
    UD_R_XMM9 = 132,
    UD_R_XMM10 = 133,
    UD_R_XMM11 = 134,
    UD_R_XMM12 = 135,
    UD_R_XMM13 = 136,
    UD_R_XMM14 = 137,
    UD_R_XMM15 = 138,
    UD_R_YMM0 = 139,
    UD_R_YMM1 = 140,
    UD_R_YMM2 = 141,
    UD_R_YMM3 = 142,
    UD_R_YMM4 = 143,
    UD_R_YMM5 = 144,
    UD_R_YMM6 = 145,
    UD_R_YMM7 = 146,
    UD_R_YMM8 = 147,
    UD_R_YMM9 = 148,
    UD_R_YMM10 = 149,
    UD_R_YMM11 = 150,
    UD_R_YMM12 = 151,
    UD_R_YMM13 = 152,
    UD_R_YMM14 = 153,
    UD_R_YMM15 = 154,
    UD_R_RIP = 155,
    UD_OP_REG = 156,
    UD_OP_MEM = 157,
    UD_OP_PTR = 158,
    UD_OP_IMM = 159,
    UD_OP_JIMM = 160,
    UD_OP_CONST = 161,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ud_lval {
    pub sbyte: i8,
    pub ubyte: u8,
    pub sword: i16,
    pub uword: u16,
    pub sdword: i32,
    pub udword: u32,
    pub sqword: i64,
    pub uqword: u64,
    pub ptr: ud_lval_ptr,
}

impl fmt::Debug for ud_lval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ud_lval {{ union }}")
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ud_lval_ptr {
    pub seg: u16,
    pub off: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ud_operand {
    pub otype: ud_type,
    pub size: u16,
    pub base: ud_type,
    pub index: ud_type,
    pub scale: u8,
    pub offset: u8,
    pub lval: ud_lval,
    pub _legacy: u64,
    pub _oprcode: u8,
}

#[repr(C)]
pub struct ud {
    //
    // Input buffering
    //
    pub inp_hook: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ud) -> libc::c_int>,
    pub inp_file: *mut libc::FILE,
    pub inp_buf: *const u8,
    pub inp_buf_size: usize,
    pub inp_buf_index: usize,
    pub inp_curr: u8,
    pub inp_ctr: usize,
    pub inp_sess: [u8; 64usize],
    pub inp_end: libc::c_int,
    pub inp_peek: libc::c_int,

    pub translator: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ud)>,
    pub insn_offset: u64,
    pub insn_hexcode: [libc::c_char; 64usize],

    //
    // Assembly output buffer
    //
    pub asm_buf: *mut libc::c_char,
    pub asm_buf_size: usize,
    pub asm_buf_fill: usize,
    pub asm_buf_int: [libc::c_char; 128usize],

    //
    // Symbol resolver for use in the translation phase.
    //
    pub sym_resolver: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ud, addr: u64, offset: *mut i64) -> *const libc::c_char>,

    pub dis_mode: u8,
    pub pc: u64,
    pub vendor: u8,
    pub mnemonic: ::itab::ud_mnemonic_code,
    pub operand: [ud_operand; 4usize],
    pub error: u8,
    pub _rex: u8,
    pub pfx_rex: u8,
    pub pfx_seg: u8,
    pub pfx_opr: u8,
    pub pfx_adr: u8,
    pub pfx_lock: u8,
    pub pfx_str: u8,
    pub pfx_rep: u8,
    pub pfx_repe: u8,
    pub pfx_repne: u8,
    pub opr_mode: u8,
    pub adr_mode: u8,
    pub br_far: u8,
    pub br_near: u8,
    pub have_modrm: u8,
    pub modrm: u8,
    pub modrm_offset: u8,
    pub vex_op: u8,
    pub vex_b1: u8,
    pub vex_b2: u8,
    pub primary_opcode: u8,
    pub user_opaque_data: *mut libc::c_void,
    pub itab_entry: *mut (),
    pub le: *mut (),
}
