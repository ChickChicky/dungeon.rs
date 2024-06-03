use std::io::{stdout,Write};

fn _style2ansi(style: u64) -> String {
    let mut s = String::new();
    s += &format!(
        "\x1b[{};{}m",
        if style & cell_style::fg::APPLY != 0 {30 + (style & 7) + (if style & 8 != 0 {60} else {0})} else {39},
        if style & cell_style::bg::APPLY != 0 {40 + ((style & (7 << 4)) >> 4) + (if style & (8 << 4) != 0 {60} else {0})} else {49}
    );
    if style & cell_style::BOLD != 0 {
        s += "\x1b[1m";
    }
    if style & cell_style::FAINT != 0 {
        s += "\x1b[2m";
    }
    if style & cell_style::ITALIC != 0 {
        s += "\x1b[3m";
    }
    if style & cell_style::REVERSE != 0 {
        s += "\x1b[7m";
    }
    if style & cell_style::STRIKE != 0 {
        s += "\x1b[9m";
    }
    return s;
}

fn stylediff2ansi(old: u64, new: u64) -> String {
    if old == new { return String::new(); }
    
    let mut s = String::new();
    
    let bgold: u32 = if old & cell_style::fg::APPLY != 0 {30 + (old & 7) as u32 + (if old & 8 != 0 {60} else {0})} else {39};
    let fgold: u32 = if old & cell_style::bg::APPLY != 0 {40 + ((old & (7 << 4)) >> 4) as u32 + (if old & (8 << 4) != 0 {60} else {0})} else {49};
    let bgnew: u32 = if new & cell_style::fg::APPLY != 0 {30 + (new & 7) as u32 + (if new & 8 != 0 {60} else {0})} else {39};
    let fgnew: u32 = if new & cell_style::bg::APPLY != 0 {40 + ((new & (7 << 4)) >> 4) as u32 + (if new & (8 << 4) != 0 {60} else {0})} else {49};

    if fgold != fgnew {
        s += "\x1b[";
        s += &fgnew.to_string();
        if bgold != bgnew {
            s += ";";
            s += &bgnew.to_string();
        }
        s += "m";
    }

    if bgold != bgnew && fgold == fgnew {
        s += &format!("\x1b[{}m",bgnew);
    }
    
    // TODO: only add a semicolon between multiple changes
    if old & cell_style::BOLD != new & cell_style::BOLD {
        s += if new & cell_style::BOLD != 0 {"\x1b[1m"} else {"\x1b[22m"};
    }
    if old & cell_style::FAINT != new & cell_style::FAINT {
        s += if new & cell_style::FAINT != 0 {"\x1b[2m"} else {"\x1b[22m"};
    }
    if old & cell_style::ITALIC != new & cell_style::ITALIC {
        s += if new & cell_style::ITALIC != 0 {"\x1b[3m"} else {"\x1b[23m"};
    }
    if old & cell_style::REVERSE != new & cell_style::REVERSE {
        s += if new & cell_style::REVERSE != 0 {"\x1b[7m"} else {"\x1b[27m"};
    }
    if old & cell_style::STRIKE != new & cell_style::STRIKE {
        s += if new & cell_style::STRIKE != 0 {"\x1b[9m"} else {"\x1b[29m"};
    }

    return s;
}

#[allow(unused)]
pub mod cell_style {
    pub mod fg {
        pub const BLACK: u64   =     APPLY; pub const LIGHT_BLACK: u64   = 8  | APPLY;
        pub const RED: u64     = 1 | APPLY; pub const LIGHT_RED: u64     = 9  | APPLY;
        pub const GREEN: u64   = 2 | APPLY; pub const LIGHT_GREEN: u64   = 10 | APPLY;
        pub const YELLOW: u64  = 3 | APPLY; pub const LIGHT_YELLOW: u64  = 11 | APPLY;
        pub const BLUE: u64    = 4 | APPLY; pub const LIGHT_BLUE: u64    = 12 | APPLY;
        pub const MAGENTA: u64 = 5 | APPLY; pub const LIGHT_MAGENTA: u64 = 13 | APPLY;
        pub const CYAN: u64    = 6 | APPLY; pub const LIGHT_CYAN: u64    = 14 | APPLY;
        pub const WHITE: u64   = 7 | APPLY; pub const LIGHT_WHITE: u64   = 15 | APPLY;

        pub const APPLY: u64 = 1 << 9;
    }

    pub mod bg {
        pub const BLACK: u64   =            APPLY; pub const LIGHT_BLACK : u64  = (8 << 4)  | APPLY;
        pub const RED: u64     = (1 << 4) | APPLY; pub const LIGHT_RED: u64     = (9 << 4)  | APPLY;
        pub const GREEN: u64   = (2 << 4) | APPLY; pub const LIGHT_GREEN: u64   = (10 << 4) | APPLY;
        pub const YELLOW: u64  = (3 << 4) | APPLY; pub const LIGHT_YELLOW: u64  = (11 << 4) | APPLY;
        pub const BLUE: u64    = (4 << 4) | APPLY; pub const LIGHT_BLUE: u64    = (12 << 4) | APPLY;
        pub const MAGENTA: u64 = (5 << 4) | APPLY; pub const LIGHT_MAGENTA: u64 = (13 << 4) | APPLY;
        pub const CYAN: u64    = (6 << 4) | APPLY; pub const LIGHT_CYAN: u64    = (14 << 4) | APPLY;
        pub const WHITE: u64   = (7 << 4) | APPLY; pub const LIGHT_WHITE: u64   = (15 << 4) | APPLY;

        pub const APPLY: u64 = 1 << 10;
    }

    pub const BOLD: u64 = 1 << 11;
    pub const FAINT: u64 = 1 << 12;
    pub const ITALIC: u64 = 1 << 13;
    pub const UNDERLINE: u64 = 1 << 14;
    pub const REVERSE: u64 = 1 << 15;
    pub const STRIKE: u64 = 1 << 16;
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub c: char,
    pub s: u64,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        return self.c == other.c && self.s == other.s;
    }
}

pub const EMPTY_CELL: Cell = Cell { c: ' ', s: 0 };

#[derive(Clone)]
pub struct Buff {
    pub cells: Vec<Cell>,
    pub width: u32,
    pub height: u32,
}

impl Buff {
    pub fn empty() -> Buff {
        let size = termsize::get().unwrap();
        Buff {
            cells: vec![EMPTY_CELL; (size.cols * size.rows) as usize],
            width: size.cols as u32,
            height: size.rows as u32,
        }
    }
}

pub struct Renderer {
    pub backbuffer: Buff,
    pub buffer: Buff,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            backbuffer: Buff::empty(),
            buffer: Buff::empty(),
        }
    }
    pub fn clear(&mut self) {
        self.buffer = Buff::empty();
    }
    pub fn flip(&mut self) {
        self.backbuffer = self.buffer.clone();
    }
    pub fn set(&mut self, x: u32, y: u32, cell: Cell) {
        self.buffer.cells[(x + y * self.buffer.width) as usize] = cell;
    }
    pub fn put(&mut self, x: u32, y: u32, text: String, style: u64) {
        let mut i = (x + y * self.buffer.width) as usize;
        for c in text.chars() {
            self.buffer.cells[i] = Cell{c,s:style};
            i += 1;
        }
    }
    pub fn fill(&mut self, x: u32, y: u32, w: u32, h: u32, cell: Cell) {
        for xx in x..x+w {
            for yy in y..y+h {
                self.buffer.cells[(xx + yy * self.buffer.width) as usize] = cell;
            }
        }
    }
    pub fn render(&mut self) {
        let mut buff = String::new();
        if self.buffer.width != self.backbuffer.width || self.buffer.height != self.backbuffer.height {
            let mut style = 0u64;
            for y in 0 .. self.buffer.height {
                for x in 0 .. self.buffer.width {
                    let cell = self.buffer.cells[(x + y * self.buffer.width) as usize];
                    buff += &stylediff2ansi(style,cell.s);
                    buff.push(cell.c);
                    style = cell.s;
                }
                if y < self.buffer.height-1 {
                    buff.push('\n');
                }
            }
        } else {
            let mut style = 0u64;
            for y in 0 .. self.buffer.height {
                let mut row = false;
                let mut streak = self.buffer.width;
                for x in 0 .. self.buffer.width {
                    let cell = self.buffer.cells[(x + y * self.buffer.width) as usize];
                    let bcell = self.backbuffer.cells[(x + y * self.buffer.width) as usize];
                    if cell != bcell {
                        if !row {
                            buff += &format!("\x1b[{};H",y+1);
                            row = true;
                        }
                        if streak+1 != x {
                            buff += &format!("\x1b[{}G",x+1);
                        }
                        streak = x;
                        buff += &stylediff2ansi(style,cell.s);
                        buff.push(cell.c);
                        style = cell.s;
                    }
                }
            }
        }
        buff += "\x1b[H\x1b[m";
        stdout().write(buff.as_bytes()).unwrap();
        stdout().flush().unwrap();
    }
}