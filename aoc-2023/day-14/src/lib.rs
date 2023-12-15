#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Landscape {
    RoundedRock,
    CubeRock,
    EmptySpace,
}

impl std::fmt::Display for Landscape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Landscape::RoundedRock => 'O',
            Landscape::CubeRock => '#',
            Landscape::EmptySpace => '.',
        };
        write!(f, "{}", symbol)
    }
}
