#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTINT0` reader - External Interrupt 0 Enable"]
pub struct EXTINT0_R(crate::FieldReader<bool, bool>);
impl EXTINT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT0` writer - External Interrupt 0 Enable"]
pub struct EXTINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `EXTINT1` reader - External Interrupt 1 Enable"]
pub struct EXTINT1_R(crate::FieldReader<bool, bool>);
impl EXTINT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT1` writer - External Interrupt 1 Enable"]
pub struct EXTINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `EXTINT2` reader - External Interrupt 2 Enable"]
pub struct EXTINT2_R(crate::FieldReader<bool, bool>);
impl EXTINT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT2` writer - External Interrupt 2 Enable"]
pub struct EXTINT2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `EXTINT3` reader - External Interrupt 3 Enable"]
pub struct EXTINT3_R(crate::FieldReader<bool, bool>);
impl EXTINT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT3` writer - External Interrupt 3 Enable"]
pub struct EXTINT3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `EXTINT4` reader - External Interrupt 4 Enable"]
pub struct EXTINT4_R(crate::FieldReader<bool, bool>);
impl EXTINT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT4` writer - External Interrupt 4 Enable"]
pub struct EXTINT4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `EXTINT5` reader - External Interrupt 5 Enable"]
pub struct EXTINT5_R(crate::FieldReader<bool, bool>);
impl EXTINT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT5` writer - External Interrupt 5 Enable"]
pub struct EXTINT5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `EXTINT6` reader - External Interrupt 6 Enable"]
pub struct EXTINT6_R(crate::FieldReader<bool, bool>);
impl EXTINT6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT6` writer - External Interrupt 6 Enable"]
pub struct EXTINT6_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `EXTINT7` reader - External Interrupt 7 Enable"]
pub struct EXTINT7_R(crate::FieldReader<bool, bool>);
impl EXTINT7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT7` writer - External Interrupt 7 Enable"]
pub struct EXTINT7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `EXTINT8` reader - External Interrupt 8 Enable"]
pub struct EXTINT8_R(crate::FieldReader<bool, bool>);
impl EXTINT8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT8` writer - External Interrupt 8 Enable"]
pub struct EXTINT8_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `EXTINT9` reader - External Interrupt 9 Enable"]
pub struct EXTINT9_R(crate::FieldReader<bool, bool>);
impl EXTINT9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT9` writer - External Interrupt 9 Enable"]
pub struct EXTINT9_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `EXTINT10` reader - External Interrupt 10 Enable"]
pub struct EXTINT10_R(crate::FieldReader<bool, bool>);
impl EXTINT10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT10` writer - External Interrupt 10 Enable"]
pub struct EXTINT10_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `EXTINT11` reader - External Interrupt 11 Enable"]
pub struct EXTINT11_R(crate::FieldReader<bool, bool>);
impl EXTINT11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT11` writer - External Interrupt 11 Enable"]
pub struct EXTINT11_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `EXTINT12` reader - External Interrupt 12 Enable"]
pub struct EXTINT12_R(crate::FieldReader<bool, bool>);
impl EXTINT12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT12` writer - External Interrupt 12 Enable"]
pub struct EXTINT12_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `EXTINT13` reader - External Interrupt 13 Enable"]
pub struct EXTINT13_R(crate::FieldReader<bool, bool>);
impl EXTINT13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT13` writer - External Interrupt 13 Enable"]
pub struct EXTINT13_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `EXTINT14` reader - External Interrupt 14 Enable"]
pub struct EXTINT14_R(crate::FieldReader<bool, bool>);
impl EXTINT14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT14` writer - External Interrupt 14 Enable"]
pub struct EXTINT14_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `EXTINT15` reader - External Interrupt 15 Enable"]
pub struct EXTINT15_R(crate::FieldReader<bool, bool>);
impl EXTINT15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT15` writer - External Interrupt 15 Enable"]
pub struct EXTINT15_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - External Interrupt 0 Enable"]
    #[inline(always)]
    pub fn extint0(&self) -> EXTINT0_R {
        EXTINT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Enable"]
    #[inline(always)]
    pub fn extint1(&self) -> EXTINT1_R {
        EXTINT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2 Enable"]
    #[inline(always)]
    pub fn extint2(&self) -> EXTINT2_R {
        EXTINT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3 Enable"]
    #[inline(always)]
    pub fn extint3(&self) -> EXTINT3_R {
        EXTINT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4 Enable"]
    #[inline(always)]
    pub fn extint4(&self) -> EXTINT4_R {
        EXTINT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5 Enable"]
    #[inline(always)]
    pub fn extint5(&self) -> EXTINT5_R {
        EXTINT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6 Enable"]
    #[inline(always)]
    pub fn extint6(&self) -> EXTINT6_R {
        EXTINT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7 Enable"]
    #[inline(always)]
    pub fn extint7(&self) -> EXTINT7_R {
        EXTINT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - External Interrupt 8 Enable"]
    #[inline(always)]
    pub fn extint8(&self) -> EXTINT8_R {
        EXTINT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - External Interrupt 9 Enable"]
    #[inline(always)]
    pub fn extint9(&self) -> EXTINT9_R {
        EXTINT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - External Interrupt 10 Enable"]
    #[inline(always)]
    pub fn extint10(&self) -> EXTINT10_R {
        EXTINT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External Interrupt 11 Enable"]
    #[inline(always)]
    pub fn extint11(&self) -> EXTINT11_R {
        EXTINT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - External Interrupt 12 Enable"]
    #[inline(always)]
    pub fn extint12(&self) -> EXTINT12_R {
        EXTINT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - External Interrupt 13 Enable"]
    #[inline(always)]
    pub fn extint13(&self) -> EXTINT13_R {
        EXTINT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - External Interrupt 14 Enable"]
    #[inline(always)]
    pub fn extint14(&self) -> EXTINT14_R {
        EXTINT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - External Interrupt 15 Enable"]
    #[inline(always)]
    pub fn extint15(&self) -> EXTINT15_R {
        EXTINT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Enable"]
    #[inline(always)]
    pub fn extint0(&mut self) -> EXTINT0_W {
        EXTINT0_W { w: self }
    }
    #[doc = "Bit 1 - External Interrupt 1 Enable"]
    #[inline(always)]
    pub fn extint1(&mut self) -> EXTINT1_W {
        EXTINT1_W { w: self }
    }
    #[doc = "Bit 2 - External Interrupt 2 Enable"]
    #[inline(always)]
    pub fn extint2(&mut self) -> EXTINT2_W {
        EXTINT2_W { w: self }
    }
    #[doc = "Bit 3 - External Interrupt 3 Enable"]
    #[inline(always)]
    pub fn extint3(&mut self) -> EXTINT3_W {
        EXTINT3_W { w: self }
    }
    #[doc = "Bit 4 - External Interrupt 4 Enable"]
    #[inline(always)]
    pub fn extint4(&mut self) -> EXTINT4_W {
        EXTINT4_W { w: self }
    }
    #[doc = "Bit 5 - External Interrupt 5 Enable"]
    #[inline(always)]
    pub fn extint5(&mut self) -> EXTINT5_W {
        EXTINT5_W { w: self }
    }
    #[doc = "Bit 6 - External Interrupt 6 Enable"]
    #[inline(always)]
    pub fn extint6(&mut self) -> EXTINT6_W {
        EXTINT6_W { w: self }
    }
    #[doc = "Bit 7 - External Interrupt 7 Enable"]
    #[inline(always)]
    pub fn extint7(&mut self) -> EXTINT7_W {
        EXTINT7_W { w: self }
    }
    #[doc = "Bit 8 - External Interrupt 8 Enable"]
    #[inline(always)]
    pub fn extint8(&mut self) -> EXTINT8_W {
        EXTINT8_W { w: self }
    }
    #[doc = "Bit 9 - External Interrupt 9 Enable"]
    #[inline(always)]
    pub fn extint9(&mut self) -> EXTINT9_W {
        EXTINT9_W { w: self }
    }
    #[doc = "Bit 10 - External Interrupt 10 Enable"]
    #[inline(always)]
    pub fn extint10(&mut self) -> EXTINT10_W {
        EXTINT10_W { w: self }
    }
    #[doc = "Bit 11 - External Interrupt 11 Enable"]
    #[inline(always)]
    pub fn extint11(&mut self) -> EXTINT11_W {
        EXTINT11_W { w: self }
    }
    #[doc = "Bit 12 - External Interrupt 12 Enable"]
    #[inline(always)]
    pub fn extint12(&mut self) -> EXTINT12_W {
        EXTINT12_W { w: self }
    }
    #[doc = "Bit 13 - External Interrupt 13 Enable"]
    #[inline(always)]
    pub fn extint13(&mut self) -> EXTINT13_W {
        EXTINT13_W { w: self }
    }
    #[doc = "Bit 14 - External Interrupt 14 Enable"]
    #[inline(always)]
    pub fn extint14(&mut self) -> EXTINT14_W {
        EXTINT14_W { w: self }
    }
    #[doc = "Bit 15 - External Interrupt 15 Enable"]
    #[inline(always)]
    pub fn extint15(&mut self) -> EXTINT15_W {
        EXTINT15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
