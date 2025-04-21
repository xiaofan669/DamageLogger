
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(static_mut_refs)]

macro_rules! subscribe_function {
    (
        $detour:ident,
        $target:expr,
        $reroute:ident
    ) => {
        $detour.initialize(std::mem::transmute($target), $reroute)?;
        $detour.enable()?;
    };
}

pub mod battle;
pub mod directx;
