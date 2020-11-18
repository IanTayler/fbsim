// TODO: move this to configuration files?
pub const SCREEN_WIDTH: f32 = 256.0;
pub const SCREEN_HEIGHT: f32 = 256.0;
pub const MAXIMUM_TEAM_SIZE: usize = 5;
pub const FORWARD_NUMBER: usize = 0;
pub const GOALIE_NUMBER: usize = 1;
pub const LEFT_NUMBER: usize = 2;
pub const RIGHT_NUMBER: usize = 3;
pub const DEFENDER_NUMBER: usize = 4;
pub const RESET_PLAYER_POSITIONS: [[f32; 2]; MAXIMUM_TEAM_SIZE] = [
    [SCREEN_WIDTH / 2.0, 4.0 * SCREEN_HEIGHT / 9.0],
    [SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 16.0],
    [SCREEN_WIDTH / 4.0, SCREEN_HEIGHT / 3.0],
    [3.0 * SCREEN_WIDTH / 4.0, SCREEN_HEIGHT / 3.0],
    [SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 4.0],
];
