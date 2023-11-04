use std::mem;
#[derive(Debug)]
#[repr(i32)]
pub enum MapType {
    MapNone = 0,
    MapSan = -1,
    MapScr = 1,
    MapMat = 2,
    MapFac = 3,
    MapRes = 4,
    MapAcc = 5,
    MapSur = 6,
    MapMin = 7,
    MapExi = 8,
    MapSto = 9,
    MapRec = 10,
    MapWas = 11,
    MapGar = 12,
    MapDsf = 13,
    MapSub = 14,
    MapLow = 15,
    MapUpp = 16,
    MapPro = 17,
    MapDee = 18,
    MapZio = 19,
    MapDat = 20,
    MapZhi = 21,
    MapWar = 22,
    MapExt = 23,
    MapCet = 24,
    MapArc = 25,
    MapHub = 26,
    MapArm = 27,
    MapLab = 28,
    MapQua = 29,
    MapTes = 30,
    MapSec = 31,
    MapCom = 32,
    MapAc0 = 33,
    MapLai = 34,
    MapTow = 35,
    MapW00 = 1000,
    MapW01 = 1001,
    MapW02 = 1002,
    MapW03 = 1003,
    MapW04 = 1004,
    MapW05 = 1005,
    MapW06 = 1006,
    MapW07 = 1007,
    MapW08 = 1008,
}

pub struct InvalidMapType(i32);

impl TryFrom<i32> for MapType {
    type Error = InvalidMapType;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::MapNone),
            -1 => Ok(Self::MapSan),
            1 => Ok(Self::MapScr),
            2 => Ok(Self::MapMat),
            3 => Ok(Self::MapFac),
            4 => Ok(Self::MapRes),
            5 => Ok(Self::MapAcc),
            6 => Ok(Self::MapSur),
            7 => Ok(Self::MapMin),
            8 => Ok(Self::MapExi),
            9 => Ok(Self::MapSto),
            10 => Ok(Self::MapRec),
            11 => Ok(Self::MapWas),
            12 => Ok(Self::MapGar),
            13 => Ok(Self::MapDsf),
            14 => Ok(Self::MapSub),
            15 => Ok(Self::MapLow),
            16 => Ok(Self::MapUpp),
            17 => Ok(Self::MapPro),
            18 => Ok(Self::MapDee),
            19 => Ok(Self::MapZio),
            20 => Ok(Self::MapDat),
            21 => Ok(Self::MapZhi),
            22 => Ok(Self::MapWar),
            23 => Ok(Self::MapExt),
            24 => Ok(Self::MapCet),
            25 => Ok(Self::MapArc),
            26 => Ok(Self::MapHub),
            27 => Ok(Self::MapArm),
            28 => Ok(Self::MapLab),
            29 => Ok(Self::MapQua),
            30 => Ok(Self::MapTes),
            31 => Ok(Self::MapSec),
            32 => Ok(Self::MapCom),
            33 => Ok(Self::MapAc0),
            34 => Ok(Self::MapLai),
            35 => Ok(Self::MapTow),
            1000 => Ok(Self::MapW00),
            1001 => Ok(Self::MapW01),
            1002 => Ok(Self::MapW02),
            1003 => Ok(Self::MapW03),
            1004 => Ok(Self::MapW04),
            1005 => Ok(Self::MapW05),
            1006 => Ok(Self::MapW06),
            1007 => Ok(Self::MapW07),
            1008 => Ok(Self::MapW08),
            _ => Err(InvalidMapType(value)),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct LuigiMachineHacking {
    pub action_ready: i32,
    pub detect_chance: i32,
    pub trace_progress: i32,
    pub last_hack_success: bool,
}
impl From<&Vec<u8>> for LuigiMachineHacking {
    fn from(slice: &Vec<u8>) -> Self {
        let p: *const [u8; mem::size_of::<Self>()] =
            slice.as_ptr() as *const [u8; mem::size_of::<Self>()];
        unsafe { mem::transmute(*p) }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct LuigiProp {
    pub id: i32,
    pub interactive_piece: bool,
}
impl From<&Vec<u8>> for LuigiProp {
    fn from(slice: &Vec<u8>) -> Self {
        let p: *const [u8; mem::size_of::<Self>()] =
            slice.as_ptr() as *const [u8; mem::size_of::<Self>()];
        unsafe { mem::transmute(*p) }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct LuigiItem {
    pub id: i32,
    pub integrity: i32,
    pub equipped: bool,
}
impl From<&Vec<u8>> for LuigiItem {
    fn from(slice: &Vec<u8>) -> Self {
        let p: *const [u8; mem::size_of::<Self>()] =
            slice.as_ptr() as *const [u8; mem::size_of::<Self>()];
        unsafe { mem::transmute(*p) }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct LuigiEntity {
    pub id: i32,
    pub integrity: i32,
    pub relation: i32,
    pub active_state: i32,
    pub exposure: i32,
    pub energy: i32,
    pub matter: i32,
    pub heat: i32,
    pub system_corruption: i32,
    pub speed: i32,
    pub inventory_size: i32,
    pub inventory: u32,
}
impl From<&Vec<u8>> for LuigiEntity {
    fn from(slice: &Vec<u8>) -> Self {
        let p: *const [u8; mem::size_of::<Self>()] =
            slice.as_ptr() as *const [u8; mem::size_of::<Self>()];
        unsafe { mem::transmute(*p) }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct LuigiTile {
    pub last_action: i32,
    pub last_fov: i32,
    pub cell: i32,
    pub door_open: bool,
    pub prop: u32,
    pub entity: u32,
    pub item: u32,
}
impl From<&Vec<u8>> for LuigiTile {
    fn from(slice: &Vec<u8>) -> Self {
        let p: *const [u8; mem::size_of::<Self>()] =
            slice.as_ptr() as *const [u8; mem::size_of::<Self>()];
        unsafe { mem::transmute(*p) }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct LuigiAi {
    pub magic1: i32,
    pub magic2: i32,
    pub action_ready: i32,
    pub map_width: i32,
    pub map_height: i32,
    pub location_depth: i32,
    pub location_map: i32,
    pub map_data: u32,
    pub map_cursor_index: i32,
    pub player: u32,
    pub machine_hacking: u32,
}
impl From<&Vec<u8>> for LuigiAi {
    fn from(slice: &Vec<u8>) -> Self {
        let p: *const [u8; mem::size_of::<Self>()] =
            slice.as_ptr() as *const [u8; mem::size_of::<Self>()];
        unsafe { mem::transmute(*p) }
    }
}
