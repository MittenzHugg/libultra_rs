use std::fmt;
use std::ops::{Deref, DerefMut};
use std::convert::{From, TryFrom};

#[allow(non_camel_case_types)]
pub enum F3dexCmd {
    G_SPNOOP = 0,
    //G_MTX = 1,
    
    //G_MOVEMEM = 0x3,
    G_VTX = 0x4,

    G_DL = 0x6,
    
    //G_SPRITED2D_BASE = 0x9,

    // G_LOAD_UCODE = 0xAF,
    // G_BRANCH_Z = 0xB0,
    G_TRI2 = 0xB1,
    // G_MODIFYVTX = 0xB2,
    // G_RDPHALF_2 = 0xB3,
    // G_RDPHALF_1 = 0xB4,
    G_LINE3D = 0xB5,
    G_CLEARGEOMETRYMODE = 0xB6,
    G_SETGEOMETRYMODE = 0xB7,
    G_ENDDL = 0xB8,
    // G_SETOTHERMODE_L = 0xB9,
    G_SETOTHERMODE_H = 0xBA,
    G_TEXTURE = 0xBB,
    // G_MOVEWORD = 0xBC,
    // G_POPMTX = 0xBD,
    // G_CULLDL = 0xBE,
    G_TRI1 = 0xBF,
    G_NOOP = 0xC0,

    G_RDPLOADSYNC = 0xE6,
    G_RDPPIPESYNC = 0xE7,
    G_RDPTILESYNC = 0xE8,
    G_RDPFULLSYNC = 0xE9,

    G_LOADTLUT = 0xF0,

    G_SETTILESIZE = 0xF2,
    G_LOADBLOCK = 0xF3,

    G_SETTILE = 0xF5,

    G_SETCOMBINE = 0xfc,


    G_SETTIMG = 0xFD,
}

impl TryFrom<u8> for F3dexCmd {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == F3dexCmd::G_SPNOOP as u8              => Ok(F3dexCmd::G_SPNOOP),
            x if x == F3dexCmd::G_VTX as u8                 => Ok(F3dexCmd::G_VTX),
            x if x == F3dexCmd::G_DL as u8                  => Ok(F3dexCmd::G_DL),
            x if x == F3dexCmd::G_TRI2 as u8                => Ok(F3dexCmd::G_TRI2),
            x if x == F3dexCmd::G_LINE3D as u8              => Ok(F3dexCmd::G_LINE3D),
            x if x == F3dexCmd::G_ENDDL as u8               => Ok(F3dexCmd::G_ENDDL),
            x if x == F3dexCmd::G_SETOTHERMODE_H as u8      => Ok(F3dexCmd::G_SETOTHERMODE_H),
            x if x == F3dexCmd::G_TEXTURE as u8             => Ok(F3dexCmd::G_TEXTURE),
            x if x == F3dexCmd::G_CLEARGEOMETRYMODE as u8   => Ok(F3dexCmd::G_CLEARGEOMETRYMODE),
            x if x == F3dexCmd::G_SETGEOMETRYMODE as u8     => Ok(F3dexCmd::G_SETGEOMETRYMODE),
            x if x == F3dexCmd::G_TRI1 as u8                => Ok(F3dexCmd::G_TRI1),
            x if x == F3dexCmd::G_NOOP as u8                => Ok(F3dexCmd::G_NOOP),
            x if x == F3dexCmd::G_RDPLOADSYNC as u8         => Ok(F3dexCmd::G_RDPLOADSYNC),
            x if x == F3dexCmd::G_RDPPIPESYNC as u8         => Ok(F3dexCmd::G_RDPPIPESYNC),
            x if x == F3dexCmd::G_RDPTILESYNC as u8         => Ok(F3dexCmd::G_RDPTILESYNC),
            x if x == F3dexCmd::G_RDPFULLSYNC as u8         => Ok(F3dexCmd::G_RDPFULLSYNC),
            x if x == F3dexCmd::G_LOADTLUT as u8            => Ok(F3dexCmd::G_LOADTLUT),
            x if x == F3dexCmd::G_SETTILESIZE as u8         => Ok(F3dexCmd::G_SETTILESIZE),
            x if x == F3dexCmd::G_LOADBLOCK as u8           => Ok(F3dexCmd::G_LOADBLOCK),
            x if x == F3dexCmd::G_SETTILE as u8             => Ok(F3dexCmd::G_SETTILE),
            x if x == F3dexCmd::G_SETCOMBINE as u8          => Ok(F3dexCmd::G_SETCOMBINE),
            x if x == F3dexCmd::G_SETTIMG as u8             => Ok(F3dexCmd::G_SETTIMG),
            _ => Err("Unknown F3Dex Command Value!")?,
        }
    }
}



impl TryFrom<u64> for F3dexCmd { 
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: u64) -> Result<Self, Self::Error> { Self::try_from(u8::try_from(value)?) }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum G_IM_FMT {
    G_IM_FMT_RGBA = 0,
    G_IM_FMT_YUV = 1,
    G_IM_FMT_CI = 2,
    G_IM_FMT_IA = 3,
    G_IM_FMT_I = 4,
}

impl TryFrom<u8> for G_IM_FMT {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == G_IM_FMT::G_IM_FMT_RGBA as u8 => Ok(G_IM_FMT::G_IM_FMT_RGBA),
            x if x == G_IM_FMT::G_IM_FMT_YUV as u8  => Ok(G_IM_FMT::G_IM_FMT_YUV),
            x if x == G_IM_FMT::G_IM_FMT_CI as u8   => Ok(G_IM_FMT::G_IM_FMT_CI),
            x if x == G_IM_FMT::G_IM_FMT_IA as u8   => Ok(G_IM_FMT::G_IM_FMT_IA),
            x if x == G_IM_FMT::G_IM_FMT_I as u8    => Ok(G_IM_FMT::G_IM_FMT_I),
            _ => Err("Unknown Image Format Value!")?,
        }
    }
}

impl TryFrom<u64> for G_IM_FMT { 
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: u64) -> Result<Self, Self::Error> { Self::try_from(u8::try_from(value)?) }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum G_IM_SIZ {
    G_IM_SIZ_4b = 0,
    G_IM_SIZ_8b = 1,
    G_IM_SIZ_16b = 2,
    G_IM_SIZ_32b = 3,
    G_IM_SIZ_DD = 5,
}

impl TryFrom<u8> for G_IM_SIZ {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == G_IM_SIZ::G_IM_SIZ_4b as u8  => Ok(G_IM_SIZ::G_IM_SIZ_4b),
            x if x == G_IM_SIZ::G_IM_SIZ_8b as u8  => Ok(G_IM_SIZ::G_IM_SIZ_8b),
            x if x == G_IM_SIZ::G_IM_SIZ_16b as u8 => Ok(G_IM_SIZ::G_IM_SIZ_16b),
            x if x == G_IM_SIZ::G_IM_SIZ_32b as u8 => Ok(G_IM_SIZ::G_IM_SIZ_32b),
            x if x == G_IM_SIZ::G_IM_SIZ_DD as u8  => Ok(G_IM_SIZ::G_IM_SIZ_DD),
            _ => Err("Unknown Image Size Value!")?,
        }
    }
}

impl TryFrom<u64> for G_IM_SIZ { 
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: u64) -> Result<Self, Self::Error> { Self::try_from(u8::try_from(value)?) }
}

#[allow(non_camel_case_types)]
enum GDLPush{
    G_DL_PUSH = 0,
    G_DL_NOPUSH = 1,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum F3dex {
    gsSPNoOp,
    gsSPVertex{v :u64, n: u64, v0: u64},
    gsSPDisplayList{dl: u64},
    gsSPBranchList{dl: u64},
    gsSP1Triangle{v: [u64; 3], flag: u64},
    gsSPLine3D{v0 : u64, v1 : u64, flag: u64},
    gsSPLineW3D{v0 : u64, v1 : u64, wd: u64, flag: u64},
    gsSPClearGeometryMode(u64),
    gsSPSetGeometryMode(u64),
    gsSPEndDisplayList,
    gsDPSetTextureLUT(u64),
    gsSPTexture{s: u64, t: u64, level: u64, tile: u64, on: u64},
    gsSPTextureL{s: u64, t: u64, level: u64, xparam: u64, tile: u64, on: u64},
    gsSP2Triangles{v0: [u64; 3], flag0: u64, v1: [u64; 3], flag1: u64},
    gsDPNoOp,
    gsDPLoadSync,
    gsDPPipeSync,
    gsDPTileSync,
    gsDPFullSync,
    gsDPLoadTLUTCmd{tile: u64, count: u64},
    gsDPSetTileSize{t: u64, uls: u64, ult: u64, lrs: u64, lrt: u64},
    gsDPLoadBlock{tile: u64, uls: u64, ult: u64, lrs: u64, dxt: u64},
    gsDPSetTile{fmt: G_IM_FMT, siz: G_IM_SIZ, line: u64, tmem: u64, tile: u64, palette: u64, 
        cmt: u64, maskt: u64, shiftt: u64, cms: u64, masks: u64, shifts: u64
    },
    gsDPSetCombine{muxs0: u64, muxs1: u64},
    gsDPSetTextureImage{fmt: G_IM_FMT, siz: G_IM_SIZ, width: u64, i: u64},
    UnknownGfx(u64),
}

#[allow(non_snake_case)]
fn _SHIFTL(v : u64, s: usize, w: usize)->u64{
    return (v & ((1 << w) - 1)) << s
}

#[allow(non_snake_case)]
fn _SHIFTR(v : u64, s: usize, w: usize)->u64{
    return (v >> s) & ((1 << w) - 1)
}

#[allow(non_snake_case)]
fn gsDPNoParam(c: F3dexCmd)->u64{ _SHIFTL(c as u64, 0x38, 8) }


#[allow(non_snake_case)]
fn gsDma1p(c: F3dexCmd, s: u64, l: u64, p: u64) -> u64{
    return _SHIFTL(c as u64, 0x38, 8) | _SHIFTL(p, 0x30, 8) | _SHIFTL(l, 0x20, 16) | _SHIFTL(s, 0, 32)
}

#[allow(non_snake_case)]
fn __gsSP1Triangle_w1(v0: u64, v1: u64, v2: u64) -> u64 {
    return _SHIFTL((v0)*2,16,8)|_SHIFTL((v1)*2,8,8)|_SHIFTL((v2)*2,0,8)
}

#[allow(non_snake_case)]
fn __gsSP1Triangle_w1f(v: [u64;3], flag: u64) -> u64{
    return match flag {
        0 => __gsSP1Triangle_w1(v[0], v[1], v[2]),
        1 => __gsSP1Triangle_w1(v[1], v[2], v[0]),
        _ => __gsSP1Triangle_w1(v[2], v[0], v[1])
    }
}

#[allow(non_snake_case)]
fn __gsSPLine3D_w1(v0: u64, v1: u64, wd: u64) -> u64 {
    return _SHIFTL((v0)*2,16,8)|_SHIFTL((v1)*2,8,8)|_SHIFTL(wd,0,8)
}

#[allow(non_snake_case)]
fn __gsSPLine3D_w1f(v0: u64, v1: u64, wd: u64, flag: u64) -> u64{
    return match flag {
        0 => __gsSPLine3D_w1(v0, v1, wd),
        _ => __gsSPLine3D_w1(v1, v0, wd)
    }
}


#[allow(non_snake_case)]
fn gsSetImage(cmd: F3dexCmd, fmt: G_IM_FMT, siz: G_IM_SIZ, width: u64, i: u64) -> u64{
    return _SHIFTL(cmd as u64, 0x38, 8) | _SHIFTL(fmt as u64, 0x35, 3) | _SHIFTL(siz as u64, 0x33, 2) | _SHIFTL((width)-1, 0x20, 12) | _SHIFTL(i, 0, 32)
}

#[allow(non_snake_case)]
fn gsDPLoadTileGeneric(c: F3dexCmd, tile: u64, uls : u64, ult: u64, lrs: u64, lrt: u64)-> u64{
    return _SHIFTL(c as u64, 0x38, 8) | _SHIFTL(uls, 0x2C, 12) | _SHIFTL(ult, 0x20, 12) | 
    _SHIFTL(tile, 0x18, 3) | _SHIFTL(lrs, 0xC, 12) | _SHIFTL(lrt, 0, 12)
}
#[allow(non_snake_case)]
fn gsSPSetOtherMode(cmd: F3dexCmd, sft: u64, len: u64, data: u64)->u64{
    _SHIFTL(cmd as u64, 0x38, 8) | _SHIFTL(sft, 0x28, 8) | _SHIFTL(len, 0x20, 8) | _SHIFTL(data, 0, 0x20)
}

impl From<u64> for F3dex {
    fn from(w: u64) -> Self {
        match F3dexCmd::try_from(_SHIFTR(w, 0x38, 8)){
            Ok(cmd) => match cmd {
                F3dexCmd::G_SPNOOP => F3dex::gsSPNoOp,
                F3dexCmd::G_VTX if _SHIFTR(w, 0x2A, 6) == (_SHIFTR(w, 0x20, 10)+1)/0x10 => F3dex::gsSPVertex{v : _SHIFTR(w, 0, 0x20), n: _SHIFTR(w, 0x2A, 6), v0: _SHIFTR(w, 0x30, 8)},
                F3dexCmd::G_DL => match _SHIFTR(w, 0x30, 8){
                    x if x == GDLPush::G_DL_PUSH as u64 => F3dex::gsSPDisplayList{dl: _SHIFTR(w, 0, 0x20)},
                    x if x == GDLPush::G_DL_NOPUSH as u64 => F3dex::gsSPBranchList{dl: _SHIFTR(w, 0, 0x20)},
                    _ => F3dex::UnknownGfx(w),
                },
                F3dexCmd::G_TRI1 => F3dex::gsSP1Triangle{v: [_SHIFTR(w, 0x10, 8)/2, _SHIFTR(w, 0x8, 8)/2, _SHIFTR(w, 0, 8)/2], flag: 0},
                F3dexCmd::G_LINE3D => {
                    let wd = _SHIFTR(w, 0, 8);
                    match wd {
                        0 => F3dex::gsSPLine3D{v0: _SHIFTR(w, 0x10, 8)/2, v1: _SHIFTR(w, 0x8, 8)/2, flag: 0},
                        _ => F3dex::gsSPLineW3D{v0: _SHIFTR(w, 0x10, 8)/2, v1: _SHIFTR(w, 0x8, 8)/2, wd, flag: 0},
                    }
                },
                F3dexCmd::G_CLEARGEOMETRYMODE => F3dex::gsSPClearGeometryMode(_SHIFTR(w, 0, 0x20)),
                F3dexCmd::G_SETGEOMETRYMODE => F3dex::gsSPSetGeometryMode(_SHIFTR(w, 0, 0x20)),
                F3dexCmd::G_ENDDL => F3dex::gsSPEndDisplayList,
                F3dexCmd::G_SETOTHERMODE_H =>{
                    let sft = _SHIFTR(w, 0x28, 8);
                    let len = _SHIFTR(w, 0x20, 8);
                    match (sft, len) {
                        /* G_MDSFT_TEXTLUT */ (14, 2) => F3dex::gsDPSetTextureLUT(_SHIFTR(w, 0, 0x20)),
                        _ => F3dex::UnknownGfx(w),
                    }
                },
                F3dexCmd::G_TEXTURE => {
                    let xparam = _SHIFTR(w, 0x30, 8);
                    let s = _SHIFTR(w, 0x10, 0x10);
                    let t = _SHIFTR(w, 0x0, 0x10);
                    let level = _SHIFTR(w, 0x2B, 0x3);
                    let tile = _SHIFTR(w, 0x28, 0x3);
                    let on = _SHIFTR(w, 0x20, 8);
                    match xparam {
                        0 => F3dex::gsSPTexture{s, t, level, tile, on},
                        _ => F3dex::gsSPTextureL{s, t, level, xparam, tile, on},
                    }
                },
                F3dexCmd::G_TRI2 => F3dex::gsSP2Triangles{ v0: [_SHIFTR(w, 0x30, 8)/2, _SHIFTR(w, 0x28, 8)/2, _SHIFTR(w, 0x20, 8)/2], flag0: 0 , v1: [_SHIFTR(w, 0x10, 8)/2, _SHIFTR(w, 0x8, 8)/2, _SHIFTR(w, 0x0, 8)/2], flag1: 0},
                F3dexCmd::G_NOOP => F3dex::gsDPNoOp,
                F3dexCmd::G_RDPLOADSYNC => F3dex::gsDPLoadSync,
                F3dexCmd::G_RDPPIPESYNC => F3dex::gsDPPipeSync,
                F3dexCmd::G_RDPTILESYNC => F3dex::gsDPTileSync,
                F3dexCmd::G_RDPFULLSYNC => F3dex::gsDPFullSync,
                F3dexCmd::G_LOADTLUT => F3dex::gsDPLoadTLUTCmd{tile: _SHIFTR(w, 24, 3), count: _SHIFTR(w, 14, 10)},
                F3dexCmd::G_SETTILESIZE => F3dex::gsDPSetTileSize{
                    t: _SHIFTR(w, 0x18, 3),
                    uls: _SHIFTR(w, 0x2C, 12), ult: _SHIFTR(w, 0x20, 12),
                    lrs: _SHIFTR(w, 0xC, 12),  lrt: _SHIFTR(w, 0, 12)
                },
                F3dexCmd::G_LOADBLOCK => F3dex::gsDPLoadBlock{tile: _SHIFTR(w, 0x18, 3), uls: _SHIFTR(w, 0x2C, 12), ult: _SHIFTR(w, 0x20, 12), lrs: _SHIFTR(w, 12, 12), dxt: _SHIFTR(w, 0, 12)},
                F3dexCmd::G_SETTILE => F3dex::gsDPSetTile{
                    fmt: G_IM_FMT::try_from(_SHIFTR(w, 0x35, 3)).unwrap(), 
                    siz: G_IM_SIZ::try_from(_SHIFTR(w, 0x33, 2)).unwrap(), 
                    line: _SHIFTR(w, 0x29, 9), 
                    tmem: _SHIFTR(w, 0x29, 9), 
                    tile: _SHIFTR(w, 24, 3),
                    palette: _SHIFTR(w, 20, 4),
                    cmt: _SHIFTR(w, 18, 2), maskt: _SHIFTR(w, 14, 4), shiftt: _SHIFTR(w, 10, 4), 
                    cms: _SHIFTR(w, 8, 2),  masks: _SHIFTR(w, 4, 4),  shifts: _SHIFTR(w, 0, 4), 
                },
                F3dexCmd::G_SETCOMBINE => F3dex::gsDPSetCombine{muxs0: _SHIFTR(w, 0x20, 0x18), muxs1: _SHIFTR(w, 0x0, 0x20)},
                F3dexCmd::G_SETTIMG => F3dex::gsDPSetTextureImage{fmt: G_IM_FMT::try_from(_SHIFTR(w, 0x35, 3)).unwrap(), siz: G_IM_SIZ::try_from(_SHIFTR(w, 0x33, 2)).unwrap(), width: _SHIFTR(w, 0x20, 12) + 1, i: _SHIFTR(w, 0, 32)},
                _ => F3dex::UnknownGfx(w),
            },
            _ => F3dex::UnknownGfx(w),
        }
    }
}

impl From<F3dex> for u64 {
    fn from(gfx: F3dex) -> u64 {
        match gfx {
            F3dex::gsSPNoOp                                 => gsDPNoParam(F3dexCmd::G_SPNOOP),
            F3dex::gsSPVertex{v, n, v0}                     => gsDma1p(F3dexCmd::G_VTX, v, _SHIFTL(n, 10, 6) | _SHIFTL(n*0x10 - 1, 0, 10), v0),
            F3dex::gsSPDisplayList{dl}                      => gsDma1p(F3dexCmd::G_DL, dl, 0, GDLPush::G_DL_PUSH as u64),
            F3dex::gsSPBranchList{dl}                       => gsDma1p(F3dexCmd::G_DL, dl, 0, GDLPush::G_DL_NOPUSH as u64),
            F3dex::gsSP1Triangle{v, flag}                   => _SHIFTL(F3dexCmd::G_TRI1 as u64, 0x38, 8) | __gsSP1Triangle_w1f(v, flag),
            F3dex::gsSPLine3D{v0, v1, flag}                 => _SHIFTL(F3dexCmd::G_LINE3D as u64, 0x38, 8) | __gsSPLine3D_w1f(v0, v1, 0, flag),
            F3dex::gsSPLineW3D{v0, v1, wd, flag}            => _SHIFTL(F3dexCmd::G_LINE3D as u64, 0x38, 8) | __gsSPLine3D_w1f(v0, v1, wd, flag),
            F3dex::gsSPClearGeometryMode(word)              => _SHIFTL(F3dexCmd::G_CLEARGEOMETRYMODE as u64, 0x38, 8) | word,
            F3dex::gsSPSetGeometryMode(word)                => _SHIFTL(F3dexCmd::G_SETGEOMETRYMODE as u64, 0x38, 8) | word,
            F3dex::gsSPEndDisplayList                       => _SHIFTL(F3dexCmd::G_ENDDL as u64, 0x38, 8),
            F3dex::gsDPSetTextureLUT(typ)                   => gsSPSetOtherMode(F3dexCmd::G_SETOTHERMODE_H, 14, 2, typ),
            F3dex::gsSPTexture{s, t, level, tile, on}       => _SHIFTL(F3dexCmd::G_TEXTURE as u64, 0x38, 8) | _SHIFTL(0, 0x30, 8) | _SHIFTL(level, 0x2B, 3) | _SHIFTL(tile, 0x28, 3) | _SHIFTL(on, 0x20, 8) | _SHIFTL(s, 0x10, 16) | _SHIFTL(t, 0, 16),
            F3dex::gsSPTextureL{s, t, level, xparam, tile, on} => _SHIFTL(F3dexCmd::G_TEXTURE as u64, 0x38, 8) | _SHIFTL(xparam, 0x30, 8) | _SHIFTL(level, 0x2B, 3) | _SHIFTL(tile, 0x28, 3) | _SHIFTL(on, 0x20, 8) | _SHIFTL(s, 0x10, 16) | _SHIFTL(t, 0, 16),
            F3dex::gsSP2Triangles{v0, flag0, v1, flag1}     => _SHIFTL(F3dexCmd::G_TRI2 as u64, 0x38, 8) | _SHIFTL(__gsSP1Triangle_w1f(v0, flag0), 0x20, 0x20) | __gsSP1Triangle_w1f(v1, flag1), 
            F3dex::gsDPNoOp                                 => gsDPNoParam(F3dexCmd::G_NOOP),
            F3dex::gsDPLoadSync                             => gsDPNoParam(F3dexCmd::G_RDPLOADSYNC),
            F3dex::gsDPPipeSync                             => gsDPNoParam(F3dexCmd::G_RDPPIPESYNC),
            F3dex::gsDPTileSync                             => gsDPNoParam(F3dexCmd::G_RDPTILESYNC),
            F3dex::gsDPFullSync                             => gsDPNoParam(F3dexCmd::G_RDPFULLSYNC),
            F3dex::gsDPLoadTLUTCmd{tile, count}             => _SHIFTL(F3dexCmd::G_LOADTLUT as u64, 0x38, 8) | _SHIFTL(tile, 24, 3) | _SHIFTL(count, 14, 10),
            F3dex::gsDPSetTileSize{t, uls, ult, lrs, lrt}   => gsDPLoadTileGeneric(F3dexCmd::G_SETTILESIZE, t, uls, ult, lrs, lrt),
            F3dex::gsDPLoadBlock{tile, uls, ult, lrs, dxt}  => _SHIFTL(F3dexCmd::G_LOADBLOCK as u64, 0x38, 8) | _SHIFTL(uls, 0x2C, 12) | _SHIFTL(ult, 0x20, 12) | _SHIFTL(tile, 0x18, 3) | _SHIFTL(lrs, 12, 12) | _SHIFTL(dxt, 0, 12),
            F3dex::gsDPSetTile{fmt, siz, line, tmem, tile, palette, 
                cmt, maskt, shiftt, cms, masks, shifts}     => _SHIFTL(_SHIFTL(F3dexCmd::G_SETTILE as u64, 24, 8) | _SHIFTL(fmt as u64, 21, 3) | _SHIFTL(siz as u64, 19, 2) | _SHIFTL(line, 9, 9) | _SHIFTL(tmem, 0, 9), 0x20, 0x20) |
                                                               _SHIFTL(tile, 24, 3) | _SHIFTL(palette, 20, 4) | _SHIFTL(cmt, 18, 2) | _SHIFTL(maskt, 14, 4) | _SHIFTL(shiftt, 10, 4) | _SHIFTL(cms, 8, 2) | _SHIFTL(masks, 4, 4) | _SHIFTL(shifts, 0, 4),
            F3dex::gsDPSetCombine{muxs0, muxs1}             => _SHIFTL(F3dexCmd::G_SETCOMBINE as u64, 0x38, 8) | _SHIFTL(muxs0, 0x20, 0x18) | _SHIFTL(muxs1, 0x0, 0x20),
            F3dex::gsDPSetTextureImage{fmt, siz, width, i}  => gsSetImage(F3dexCmd::G_SETTIMG, fmt, siz, width, i),
            F3dex::UnknownGfx(w) => w,
        }
    }
}

impl fmt::Debug for F3dex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            F3dex::gsSPNoOp                                 => f.write_str("gsSPNoOp()"),
            F3dex::gsSPVertex{v, n, v0}                     => f.write_fmt(format_args!("gsSPVertex(SEGMENT_ADDR({},0x{:X}), {}, {})", _SHIFTR(*v, 0x18, 8), _SHIFTR(*v, 0, 0x18), n, v0)),
            F3dex::gsSPDisplayList{dl}                      => f.write_fmt(format_args!("gsSPDisplayList(SEGMENT_ADDR({},0x{:X}))", _SHIFTR(*dl, 0x18, 8), _SHIFTR(*dl, 0, 0x18))),
            F3dex::gsSPBranchList{dl}                       => f.write_fmt(format_args!("gsSPBranchList(SEGMENT_ADDR({},0x{:X})", _SHIFTR(*dl, 0x18, 8), _SHIFTR(*dl, 0, 0x18))),
            F3dex::gsSP1Triangle{v, flag}                   => f.write_fmt(format_args!("gsSP1Triangle({}, {}, {}, {})", v[0], v[1], v[2], flag)),
            F3dex::gsSPLine3D{v0, v1, flag}                 => f.write_fmt(format_args!("gsSPLine3D({}, {}, {})", v0, v1, flag)),
            F3dex::gsSPLineW3D{v0, v1, wd, flag}            => f.write_fmt(format_args!("gsSPLineW3D({}, {}, {}, {})", v0, v1, wd, flag)),
            F3dex::gsSPClearGeometryMode(word)              => f.write_fmt(format_args!("gsSPClearGeometryMode(0x{:08X})", word)),
            F3dex::gsSPSetGeometryMode(word)                => f.write_fmt(format_args!("gsSPSetGeometryMode(0x{:08X})", word)),
            F3dex::gsSPEndDisplayList                       => f.write_str("gsSPEndDisplayList()"),
            F3dex::gsDPSetTextureLUT(typ)                   => write!(f, "gsDPSetTextureLUT({})", typ),
            F3dex::gsSPTexture{s, t, level, tile, on}       => write!(f, "gsSPTexture(0x{:X}, 0x{:X}, {}, {}, {})", s, t, level, tile, on),
            F3dex::gsSPTextureL{s, t, level, xparam, tile, on} => write!(f, "gsSPTextureL(0x{:X}, 0x{:X}, {}, {}, {}, {})", s, t, level, xparam, tile, on),
            F3dex::gsSP2Triangles{v0, flag0, v1, flag1}     => f.write_fmt(format_args!("gsSP2Triangles({}, {}, {}, {}, {}, {}, {}, {})", v0[0], v0[1], v0[2], flag0, v1[0], v1[1], v1[2], flag1)),
            F3dex::gsDPNoOp                                 => f.write_str("gsDPNoOp()"),
            F3dex::gsDPLoadSync                             => f.write_str("gsDPLoadSync()"),
            F3dex::gsDPPipeSync                             => f.write_str("gsDPPipeSync()"),
            F3dex::gsDPTileSync                             => f.write_str("gsDPTileSync()"),
            F3dex::gsDPFullSync                             => f.write_str("gsDPFullSync()"),
            F3dex::gsDPLoadTLUTCmd{tile, count}             => f.write_fmt(format_args!("gsDPLoadTLUTCmd({}, {})", tile, count)),
            F3dex::gsDPSetTileSize{t, uls, ult, lrs, lrt}   => f.write_fmt(format_args!("gsDPSetTileSize({}, {}, {}, {}, {})", t, uls, ult, lrs, lrt)),
            F3dex::gsDPLoadBlock{tile, uls, ult, lrs, dxt}  => f.write_fmt(format_args!("gsDPLoadBlock({}, {}, {}, {}, {})", tile, uls, ult, lrs, dxt)),
            F3dex::gsDPSetTile{fmt, siz, line, tmem, tile, palette, 
                cmt, maskt, shiftt, cms, masks, shifts}     => f.write_fmt(format_args!("gsDPSetTile({:?}, {:?}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})", fmt, siz, line, tmem, tile, palette, cmt, maskt, shiftt, cms, masks, shifts)),
            F3dex::gsDPSetCombine{muxs0, muxs1}             => write!(f, "gsDPSetCombine(0x{:X},0x{:X})", muxs0, muxs1),
            F3dex::gsDPSetTextureImage{fmt, siz, width, i}  => f.write_fmt(format_args!("gsDPSetTextureImage({:?}, {:?}, {}, SEGMENT_ADDR({}, 0x{:X}))", fmt, siz, width, _SHIFTR(*i, 0x18, 8), _SHIFTR(*i, 0, 0x18))),
            F3dex::UnknownGfx(val)                          => f.write_fmt(format_args!("0x{:016X}", val)),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vtx {
    pub ob : [i16; 3],
    pub flag : u16,
    pub tc : [i16 ;2],
    pub cn : [u8; 4],
}

impl Vtx {
    pub fn from_be_bytes(in_bytes: &[u8])->Vtx{
        let ob = [
            i16::from_be_bytes([in_bytes[0], in_bytes[1]]), 
            i16::from_be_bytes([in_bytes[2], in_bytes[3]]),
            i16::from_be_bytes([in_bytes[4], in_bytes[5]])
        ];
        let flag = u16::from_be_bytes([in_bytes[6], in_bytes[7]]);
        let tc = [
            i16::from_be_bytes([in_bytes[8], in_bytes[9]]),
            i16::from_be_bytes([in_bytes[0xa], in_bytes[0xb]]),
        ];
        let cn = in_bytes[0xc..0x10].try_into().unwrap();
        return Vtx{ob, flag, tc, cn}
    }

    pub fn to_bytes(&self)->Vec<u8>{
        let mut out: Vec<Vec<u8>> = Vec::new();
        for coord in self.ob {
            out.push(coord.to_be_bytes().to_vec());
        }
        out.push(self.flag.to_be_bytes().to_vec());
        for st in self.tc {
            out.push(st.to_be_bytes().to_vec());
        }
        out.push(self.cn.to_vec());
        return out.into_iter().flatten().collect()
    }
}

impl fmt::Debug for Vtx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vtx{{ob: {:>5?}, flag: 0x{:X}, tc: {:>6?}, cn: {:>3?}}}", &self.ob, &self.flag, &self.tc, &self.cn)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
