use bevy::prelude::*;

pub const MAX_BLOCKDATA: usize = 4;
pub const I_BLOCK: [[usize; 16]; 4] = [
    [
        0,0,0,0,
        1,2,3,4,
        0,0,0,0,
        0,0,0,0,
    ],
    [
        0,0,1,0,
        0,0,2,0,
        0,0,3,0,
        0,0,4,0,
    ],
    [
        0,0,0,0,
        0,0,0,0,
        1,2,3,4,
        0,0,0,0,
    ],
    [
        0,1,0,0,
        0,2,0,0,
        0,3,0,0,
        0,4,0,0,
    ],
];
pub const J_BLOCK: [[usize; 16]; 4] = [
    [
        1,0,0,0,
        2,3,4,0,
        0,0,0,0,
        0,0,0,0,
    ],
    [
        0,1,2,0,
        0,3,0,0,
        0,4,0,0,
        0,0,0,0,
    ],
    [
        0,0,0,0,
        1,2,3,0,
        0,0,4,0,
        0,0,0,0,
    ],
    [
        0,1,0,0,
        0,2,0,0,
        3,4,0,0,
        0,0,0,0,
    ],
];
pub const L_BLOCK: [[usize; 16]; 4] = [
    [
        0,0,1,0,
        4,3,2,0,
        0,0,0,0,
        0,0,0,0,
    ],
    [
        0,1,0,0,
        0,2,0,0,
        0,3,4,0,
        0,0,0,0,
    ],
    [
        0,0,0,0,
        1,2,3,0,
        4,0,0,0,
        0,0,0,0,
    ],
    [
        1,2,0,0,
        0,3,0,0,
        0,4,0,0,
        0,0,0,0,
    ],
];
pub const O_BLOCK: [[usize; 16]; 4] = [
    [
        0,0,0,0,
        0,1,2,0,
        0,3,4,0,
        0,0,0,0,
    ],
    [
        0,0,0,0,
        0,1,2,0,
        0,3,4,0,
        0,0,0,0,
    ],
    [
        0,0,0,0,
        0,1,2,0,
        0,3,4,0,
        0,0,0,0,
    ],
    [
        0,0,0,0,
        0,1,2,0,
        0,3,4,0,
        0,0,0,0,
    ],
];
pub const S_BLOCK: [[usize; 16]; 4] = [
    [
        0,0,0,0,
        0,1,2,0,
        3,4,0,0,
        0,0,0,0,
    ],
    [
        0,1,0,0,
        0,2,3,0,
        0,0,4,0,
        0,0,0,0,
    ],
    [
        0,2,1,0,
        3,4,0,0,
        0,0,0,0,
        0,0,0,0,
    ],
    [
        0,1,0,0,
        0,2,3,0,
        0,0,4,0,
        0,0,0,0,
    ],
];
pub const T_BLOCK: [[usize; 16]; 4] = [
    [
        0,1,0,0,
        2,3,4,0,
        0,0,0,0,
        0,0,0,0,
    ],
    [
        0,1,0,0,
        0,2,3,0,
        0,4,0,0,
        0,0,0,0,
    ],
    [
        0,0,0,0,
        1,2,3,0,
        0,4,0,0,
        0,0,0,0,
    ],
    [
        0,1,0,0,
        2,3,0,0,
        0,4,0,0,
        0,0,0,0,
    ],
];
pub const Z_BLOCK: [[usize; 16]; 4] = [
    [
        0,0,0,0,
        1,2,0,0,
        0,3,4,0,
        0,0,0,0,
    ],
    [
        0,0,1,0,
        0,2,3,0,
        0,4,0,0,
        0,0,0,0,
    ],
    [
        1,2,0,0,
        0,3,4,0,
        0,0,0,0,
        0,0,0,0,
    ],
    [
        0,0,1,0,
        0,2,3,0,
        0,4,0,0,
        0,0,0,0,
    ],
];
pub const I_COLOR: Color = Color::srgb(0.0, 0.0, 1.0);
pub const J_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);
pub const L_COLOR: Color = Color::srgb(0.0, 1.0, 1.0);
pub const O_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
pub const S_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);
pub const T_COLOR: Color = Color::srgb(1.0, 1.0, 0.0);
pub const Z_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
