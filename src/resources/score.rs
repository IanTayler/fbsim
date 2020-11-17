pub struct Score {
    pub team1: u32,
    pub team2: u32,
    pub in_goal_last_frame: bool,
}

impl Default for Score {
    fn default() -> Self {
        Score {
            team1: 0,
            team2: 0,
            in_goal_last_frame: false,
        }
    }
}
