// build-pass
// compile-flags: -C llvm-args=--disassemble
// normalize-stderr-test "OpSource .*\n" -> ""
// normalize-stderr-test "OpLine .*\n" -> ""
// normalize-stderr-test "%\d+ = OpString .*\n" -> ""
// normalize-stderr-test "^(; .*\n)*" -> ""
// normalize-stderr-test "OpCapability VulkanMemoryModel\n" -> ""
// normalize-stderr-test "OpMemoryModel Logical Vulkan" -> "OpMemoryModel Logical Simple"
// ignore-spv1.0
// ignore-spv1.1
// ignore-spv1.2
// ignore-spv1.3
// ignore-vulkan1.0
// ignore-vulkan1.1

use core::marker::PhantomData;
use spirv_std::glam::*;
use spirv_std::spirv;

#[repr(C)]
#[derive(Default)]
struct SomeStruct {
    a: u32,
    b: f32,
    c: Vec3,
}

#[derive(Default)]
pub struct A(u32);
#[repr(transparent)]
#[derive(Default)]
pub struct ATrans(u32);

#[derive(Default)]
pub struct B(Vec3);
#[repr(transparent)]
#[derive(Default)]
pub struct BTrans(Vec3);

#[derive(Default)]
pub struct C(SomeStruct);
#[repr(transparent)]
#[derive(Default)]
pub struct CTrans(SomeStruct);

#[derive(Default)]
pub struct D<'a>(u32, PhantomData<&'a ()>);
#[repr(transparent)]
#[derive(Default)]
pub struct DTrans<'a>(u32, PhantomData<&'a ()>);

#[derive(Default)]
pub struct E(ATrans);
#[repr(transparent)]
#[derive(Default)]
pub struct ETrans(ATrans);

#[derive(Default)]
pub struct Zst;
#[repr(transparent)]
#[derive(Default)]
pub struct ZstTrans;

#[spirv(vertex)]
pub fn main(
    a: &mut A,
    a_trans: &mut ATrans,
    b: &mut B,
    b_trans: &mut BTrans,
    c: &mut C,
    c_trans: &mut CTrans,
    d: &mut D<'static>,
    d_trans: &mut DTrans<'static>,
    e: &mut E,
    e_trans: &mut ETrans,
    zst: &mut Zst,
    zst_trans: &mut ZstTrans,
) {
    *a = Default::default();
    *a_trans = Default::default();
    *b = Default::default();
    *b_trans = Default::default();
    *c = Default::default();
    *c_trans = Default::default();
    *d = Default::default();
    *d_trans = Default::default();
    *e = Default::default();
    *e_trans = Default::default();
    *zst = Default::default();
    *zst_trans = Default::default();
}
