struct Flags {
    z : u8,
    s : u8,
    p : u8,
    cy : u8,
    ac : u8,

}
pub struct Intel8080 {
    a : u8,
    b : u8,
    c : u8,
    d : u8,
    e : u8,
    h : u8,
    l : u8,
    sp : u16,
    pc : u16,
    mem : [u8; 8096],
    flags : Flags
}


