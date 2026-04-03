#[derive(Debug, Clone, Default)]
pub struct SongStats {
    pub song_name: String,
    pub total_plays: u32,
    pub opener_count: u32,
    pub closer_count: u32,
    pub encore_count: u32,
    pub positions_played: Vec<usize>,
    pub mean_positions_played: Vec<f32>,
    pub last_played: String,
}
