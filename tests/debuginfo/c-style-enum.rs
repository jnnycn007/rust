//@ ignore-aarch64
//@ min-lldb-version: 310

//@ compile-flags:-g

// === GDB TESTS ===================================================================================

// gdbr-command:print c_style_enum::SINGLE_VARIANT
// gdbr-check:$1 = c_style_enum::SingleVariant::TheOnlyVariant

// gdbr-command:print c_style_enum::AUTO_ONE
// gdbr-check:$2 = c_style_enum::AutoDiscriminant::One

// gdbr-command:print c_style_enum::AUTO_TWO
// gdbr-check:$3 = c_style_enum::AutoDiscriminant::One

// gdbr-command:print c_style_enum::AUTO_THREE
// gdbr-check:$4 = c_style_enum::AutoDiscriminant::One

// gdbr-command:print c_style_enum::MANUAL_ONE
// gdbr-check:$5 = c_style_enum::ManualDiscriminant::OneHundred

// gdbr-command:print c_style_enum::MANUAL_TWO
// gdbr-check:$6 = c_style_enum::ManualDiscriminant::OneHundred

// gdbr-command:print c_style_enum::MANUAL_THREE
// gdbr-check:$7 = c_style_enum::ManualDiscriminant::OneHundred

// gdb-command:run

// gdb-command:print auto_one
// gdbr-check:$8 = c_style_enum::AutoDiscriminant::One

// gdb-command:print auto_two
// gdbr-check:$9 = c_style_enum::AutoDiscriminant::Two

// gdb-command:print auto_three
// gdbr-check:$10 = c_style_enum::AutoDiscriminant::Three

// gdb-command:print manual_one_hundred
// gdbr-check:$11 = c_style_enum::ManualDiscriminant::OneHundred

// gdb-command:print manual_one_thousand
// gdbr-check:$12 = c_style_enum::ManualDiscriminant::OneThousand

// gdb-command:print manual_one_million
// gdbr-check:$13 = c_style_enum::ManualDiscriminant::OneMillion

// gdb-command:print single_variant
// gdbr-check:$14 = c_style_enum::SingleVariant::TheOnlyVariant

// gdbr-command:print AUTO_TWO
// gdbr-check:$15 = c_style_enum::AutoDiscriminant::Two

// gdbr-command:print AUTO_THREE
// gdbr-check:$16 = c_style_enum::AutoDiscriminant::Three

// gdbr-command:print MANUAL_TWO
// gdbr-check:$17 = c_style_enum::ManualDiscriminant::OneThousand

// gdbr-command:print MANUAL_THREE
// gdbr-check:$18 = c_style_enum::ManualDiscriminant::OneMillion


// === LLDB TESTS ==================================================================================

// lldb-command:run

// lldb-command:v auto_one
// lldbg-check:[...] One
// lldbr-check:(c_style_enum::AutoDiscriminant) auto_one = c_style_enum::AutoDiscriminant::One

// lldb-command:v auto_two
// lldbg-check:[...] Two
// lldbr-check:(c_style_enum::AutoDiscriminant) auto_two = c_style_enum::AutoDiscriminant::Two

// lldb-command:v auto_three
// lldbg-check:[...] Three
// lldbr-check:(c_style_enum::AutoDiscriminant) auto_three = c_style_enum::AutoDiscriminant::Three

// lldb-command:v manual_one_hundred
// lldbg-check:[...] OneHundred
// lldbr-check:(c_style_enum::ManualDiscriminant) manual_one_hundred = c_style_enum::ManualDiscriminant::OneHundred

// lldb-command:v manual_one_thousand
// lldbg-check:[...] OneThousand
// lldbr-check:(c_style_enum::ManualDiscriminant) manual_one_thousand = c_style_enum::ManualDiscriminant::OneThousand

// lldb-command:v manual_one_million
// lldbg-check:[...] OneMillion
// lldbr-check:(c_style_enum::ManualDiscriminant) manual_one_million = c_style_enum::ManualDiscriminant::OneMillion

// lldb-command:v single_variant
// lldbg-check:[...] TheOnlyVariant
// lldbr-check:(c_style_enum::SingleVariant) single_variant = c_style_enum::SingleVariant::TheOnlyVariant

#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(omit_gdb_pretty_printer_section)]
#![omit_gdb_pretty_printer_section]

use self::AutoDiscriminant::{One, Two, Three};
use self::ManualDiscriminant::{OneHundred, OneThousand, OneMillion};
use self::SingleVariant::TheOnlyVariant;

#[derive(Copy, Clone)]
enum AutoDiscriminant {
    One,
    Two,
    Three
}

#[derive(Copy, Clone)]
enum ManualDiscriminant {
    OneHundred = 100,
    OneThousand = 1000,
    OneMillion = 1000000
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum SingleVariant {
    TheOnlyVariant
}

static SINGLE_VARIANT: SingleVariant = TheOnlyVariant;

static mut AUTO_ONE: AutoDiscriminant = One;
static mut AUTO_TWO: AutoDiscriminant = One;
static mut AUTO_THREE: AutoDiscriminant = One;

static mut MANUAL_ONE: ManualDiscriminant = OneHundred;
static mut MANUAL_TWO: ManualDiscriminant = OneHundred;
static mut MANUAL_THREE: ManualDiscriminant = OneHundred;

fn main() {

    let auto_one = One;
    let auto_two = Two;
    let auto_three = Three;

    let manual_one_hundred = OneHundred;
    let manual_one_thousand = OneThousand;
    let manual_one_million = OneMillion;

    let single_variant = TheOnlyVariant;

    unsafe {
        AUTO_TWO = Two;
        AUTO_THREE = Three;

        MANUAL_TWO = OneThousand;
        MANUAL_THREE = OneMillion;
    };

    zzz(); // #break

    // Borrow to avoid an eager load of the constant value in the static.
    let a = &SINGLE_VARIANT;
    let a = unsafe { AUTO_ONE };
    let a = unsafe { MANUAL_ONE };
}

fn zzz() { () }
