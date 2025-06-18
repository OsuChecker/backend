#[derive(Debug, Default, Clone)]
pub struct Hit{
    pub _geki:u32,
    pub _300:u32,
    pub _katu:u32,
    pub _100:u32,
    pub _50:u32,
    pub _miss:u32,
}
impl Hit {
    pub fn new(
        _geki: u32,
        _300: u32,
        _katu: u32,
        _100: u32,
        _50: u32,
        _miss: u32,
    ) -> Self {
        Self {
            _geki,
            _300,
            _katu,
            _100,
            _50,
            _miss,
        }
    }
}

pub fn calculate_accuracy(
    gamemode: u8,
    hit: Hit
) -> f64 {
    let (numerator, denominator) = match gamemode {
        0 => (
                hit._300 as f64 * 6.0 + hit._100 as f64 * 2.0 + hit._50 as f64,
                (hit._300 + hit._100 + hit._50 + hit._miss) as f64 * 6.0
        ),
        1 => (
                hit._300 as f64 * 2.0 + hit._100 as f64,
                (hit._300 + hit._100 + hit._50 + hit._miss) as f64 * 2.0
        ),
        2 => (
                (hit._300 + hit._100 + hit._50) as f64,
                (hit._300 + hit._100 + hit._50 + hit._katu + hit._miss) as f64
        ),
        3 => (
                (hit._geki + hit._300) as f64 * 6.0 + hit._katu as f64 * 4.0 + hit._100 as f64 * 2.0 + hit._50 as f64,
                (hit._geki + hit._300 + hit._katu + hit._100 + hit._50 + hit._miss) as f64 * 6.0
        ),
        _ => panic!("Unsupported Gamemode : {}", gamemode)
    };

    numerator / denominator
}

pub fn accuracy_to_rank(accuracy: f64) -> String {
    if accuracy >= 1.0 {
        "SS".to_string()
    } else if accuracy >= 0.95 {
        "S".to_string()

    } else if accuracy >= 0.9 {
        "A".to_string()
    } else if accuracy >= 0.8 {
        "B".to_string()
    } else if accuracy >= 0.7 {
        "C".to_string()
    } else {
        "D".to_string()
    }
}

