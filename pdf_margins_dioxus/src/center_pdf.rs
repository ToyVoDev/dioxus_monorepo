use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaperSize {
    Letter,
    Legal,
    Tabloid,
    Ledger,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    B4,
    B5,
    FourA0,
    TwoA0,
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    Executive,
    Folio,
    GovernmentLetter,
    JuniorLegal,
}

impl PaperSize {
    pub fn dimensions(&self) -> (f64, f64) {
        match self {
            Self::Letter => (612.0, 792.0),
            Self::Legal => (612.0, 1008.0),
            Self::Tabloid => (792.0, 1224.0),
            Self::Ledger => (1224.0, 792.0),
            Self::A0 => (2383.94, 3370.39),
            Self::A1 => (1683.78, 2383.94),
            Self::A2 => (1190.55, 1683.78),
            Self::A3 => (841.89, 1190.55),
            Self::A4 => (595.28, 841.89),
            Self::A5 => (419.53, 595.28),
            Self::A6 => (297.64, 419.53),
            Self::B4 => (708.66, 1000.63),
            Self::B5 => (498.90, 708.66),
            Self::FourA0 => (4767.87, 6740.79),
            Self::TwoA0 => (3370.39, 4767.87),
            Self::C0 => (2599.37, 3676.54),
            Self::C1 => (1836.85, 2599.37),
            Self::C2 => (1298.27, 1836.85),
            Self::C3 => (918.43, 1298.27),
            Self::C4 => (649.13, 918.43),
            Self::C5 => (459.21, 649.13),
            Self::C6 => (323.15, 459.21),
            Self::C7 => (229.61, 323.15),
            Self::C8 => (161.57, 229.61),
            Self::C9 => (113.39, 161.57),
            Self::C10 => (79.37, 113.39),
            Self::Executive => (521.86, 756.0),
            Self::Folio => (612.0, 936.0),
            Self::GovernmentLetter => (576.0, 756.0),
            Self::JuniorLegal => (360.0, 576.0),
        }
    }

    pub const ALL: &[PaperSize] = &[
        Self::Letter,
        Self::Legal,
        Self::Tabloid,
        Self::Ledger,
        Self::A0,
        Self::A1,
        Self::A2,
        Self::A3,
        Self::A4,
        Self::A5,
        Self::A6,
        Self::B4,
        Self::B5,
        Self::FourA0,
        Self::TwoA0,
        Self::C0,
        Self::C1,
        Self::C2,
        Self::C3,
        Self::C4,
        Self::C5,
        Self::C6,
        Self::C7,
        Self::C8,
        Self::C9,
        Self::C10,
        Self::Executive,
        Self::Folio,
        Self::GovernmentLetter,
        Self::JuniorLegal,
    ];
}

impl fmt::Display for PaperSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FourA0 => write!(f, "4A0"),
            Self::TwoA0 => write!(f, "2A0"),
            Self::GovernmentLetter => write!(f, "Government Letter"),
            Self::JuniorLegal => write!(f, "Junior Legal"),
            other => write!(f, "{:?}", other),
        }
    }
}

pub struct CenterOptions {
    pub paper_size: (f64, f64),
    pub draw_alignment: bool,
    pub draw_border: bool,
    pub nudge_x: f64,
    pub nudge_y: f64,
    pub nudge_border_x: f64,
    pub nudge_border_y: f64,
}

pub fn center_pdf(pdf_bytes: &[u8], options: &CenterOptions) -> Result<Vec<u8>, String> {
    // TODO: implement in Task 3
    Err("not yet implemented".to_string())
}
