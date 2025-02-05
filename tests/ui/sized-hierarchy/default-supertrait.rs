//@ check-fail
#![feature(sized_hierarchy)]
use std::marker::MetaSized;

trait PosSized: Sized { }

trait NegSized: ?Sized { }
//~^ ERROR only `?MetaSized` is permitted in supertraits

trait PosMetaSized: MetaSized { }

trait NegMetaSized: ?MetaSized { }


trait PosSizedPosMetaSized: Sized + MetaSized { }

trait PosSizedNegMetaSized: Sized + ?MetaSized { }
//~^ ERROR `?MetaSized` supertrait cannot be used with `Sized`

trait NegSizedPosMetaSized: ?Sized + MetaSized { }
//~^ ERROR only `?MetaSized` is permitted in supertraits

trait NegSizedNegMetaSized: ?Sized + ?MetaSized { }
//~^ ERROR only `?MetaSized` is permitted in supertraits

trait PosMetaSizedPosSized: MetaSized + Sized { }

trait PosMetaSizedNegSized: MetaSized + ?Sized { }
//~^ ERROR only `?MetaSized` is permitted in supertraits

trait NegMetaSizedPosSized: ?MetaSized + Sized { }
//~^ ERROR `?MetaSized` supertrait cannot be used with `Sized`

trait NegMetaSizedNegSized: ?MetaSized + ?Sized { }
//~^ ERROR only `?MetaSized` is permitted in supertraits


trait RepeatedSizedBound: ?Sized + ?Sized { }
//~^ ERROR only `?MetaSized` is permitted in supertraits
//~^^ ERROR only `?MetaSized` is permitted in supertraits
//~^^^ ERROR supertrait has more than one relaxed default supertrait, only one is supported

trait RepeatedMetaSizedBound: ?MetaSized + ?MetaSized { }
//~^ ERROR supertrait has more than one relaxed default supertrait, only one is supported

trait AdditionalRelaxedBound: ?MetaSized + ?Copy { }
//~^ ERROR only `?MetaSized` is permitted in supertraits

fn main() { }
