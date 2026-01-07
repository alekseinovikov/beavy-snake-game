pub mod window {
    pub const TITLE: &str = "Beavy Snake Game";
    pub const WIDTH: u32 = 960;
    pub const HEIGHT: u32 = 540;
}

pub mod grid {
    pub const WIDTH: i32 = 20;
    pub const HEIGHT: i32 = 15;
    pub const CELL_SIZE: f32 = 24.0;
    pub const SNAKE_SIZE: f32 = CELL_SIZE - 2.0;
    pub const FOOD_SIZE: f32 = CELL_SIZE - 4.0;
    pub const BORDER_THICKNESS: f32 = 2.0;
    pub const BORDER_Z: f32 = 0.1;
}

pub mod timing {
    pub const TICK_SECONDS: f32 = 0.18;
}

pub mod ui {
    pub const ROOT_PERCENT: f32 = 100.0;
    pub const PANEL_GAP: f32 = 12.0;
    pub const BUTTON_WIDTH: f32 = 220.0;
    pub const BUTTON_HEIGHT: f32 = 48.0;
    pub const BUTTON_BORDER: f32 = 1.0;
    pub const HUD_FONT_SIZE: f32 = 18.0;
    pub const SCORE_FONT_SIZE: f32 = 18.0;
    pub const TITLE_FONT_SIZE: f32 = 42.0;
    pub const SUBTITLE_FONT_SIZE: f32 = 22.0;
    pub const BUTTON_FONT_SIZE: f32 = 20.0;
    pub const FPS_TOP: f32 = 8.0;
    pub const FPS_LEFT: f32 = 8.0;
    pub const SCORE_TOP: f32 = 8.0;
    pub const SCORE_RIGHT: f32 = 8.0;
}

pub mod text {
    pub const MENU_TITLE: &str = "Beavy Snake Game";
    pub const MENU_START: &str = "Start Game";
    pub const MENU_EXIT: &str = "Exit";
    pub const GAME_OVER_TITLE: &str = "Game Over";
    pub const GAME_OVER_RESTART: &str = "Play Again";
    pub const GAME_OVER_EXIT: &str = "Exit";
    pub const SCORE_LABEL: &str = "Score: ";
    pub const FPS_LABEL: &str = "FPS: ";
}

pub mod colors {
    pub const WHITE: (f32, f32, f32) = (1.0, 1.0, 1.0);
    pub const MENU_BG: (f32, f32, f32) = (0.05, 0.06, 0.08);
    pub const MENU_BUTTON_BG: (f32, f32, f32) = (0.1, 0.12, 0.16);
    pub const MENU_BUTTON_BORDER: (f32, f32, f32) = (0.18, 0.2, 0.25);
    pub const GAME_OVER_BG: (f32, f32, f32) = (0.08, 0.04, 0.05);
    pub const GAME_OVER_TEXT: (f32, f32, f32) = (0.9, 0.9, 0.9);
    pub const GAME_OVER_BUTTON_BG: (f32, f32, f32) = (0.16, 0.12, 0.14);
    pub const GAME_OVER_EXIT_BG: (f32, f32, f32) = (0.12, 0.1, 0.1);
    pub const GAME_OVER_BUTTON_BORDER: (f32, f32, f32) = (0.2, 0.2, 0.25);
    pub const SNAKE: (f32, f32, f32) = (0.2, 0.9, 0.4);
    pub const FOOD: (f32, f32, f32) = (0.95, 0.3, 0.3);
    pub const BORDER: (f32, f32, f32) = (0.85, 0.85, 0.85);
}
