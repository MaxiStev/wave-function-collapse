//╦
//╩
//╣
//╠
//╔
//╚
//╗
//╝
//═
//║
use super::CellContent;
pub fn cells() -> Vec<CellContent> {
    vec![
        CellContent {
            content: ' ',
            ..Default::default()
        },
        CellContent {
            content: '╠',
            top: 2,
            right: 2,
            bottom: 2,
            ..Default::default()
        },
        CellContent {
            content: '╩',
            top: 2,
            right: 2,
            left: 2,
            ..Default::default()
        },
        CellContent {
            content: '╣',
            top: 2,
            bottom: 2,
            left: 2,
            ..Default::default()
        },
        CellContent {
            content: '╦',
            right: 2,
            bottom: 2,
            left: 2,
            ..Default::default()
        },
        CellContent {
            content: '║',
            top: 2,
            bottom: 2,
            ..Default::default()
        },
        CellContent {
            content: '═',
            right: 2,
            left: 2,
            ..Default::default()
        },
        CellContent {
            content: '╚',
            top: 2,
            right: 2,
            ..Default::default()
        },
        CellContent {
            content: '╝',
            top: 2,
            left: 2,
            ..Default::default()
        },
        CellContent {
            content: '╔',
            right: 2,
            bottom: 2,
            ..Default::default()
        },
        CellContent {
            content: '╗',
            bottom: 2,
            left: 2,
            ..Default::default()
        },
        CellContent {
            content: '╬',
            top: 2,
            right: 2,
            bottom: 2,
            left: 2,
        },
        CellContent {
            content: '│',
            top: 1,
            bottom: 1,
            ..Default::default()
        },
        CellContent {
            content: '┤',
            top: 1,
            bottom: 1,
            left: 1,
            ..Default::default()
        },
        CellContent {
            content: '╡',
            top: 1,
            bottom: 1,
            left: 2,
            ..Default::default()
        },
        CellContent {
            content: '╢',
            top: 2,
            bottom: 2,
            left: 1,
            ..Default::default()
        },
        CellContent {
            content: '╖',
            bottom: 2,
            left: 1,
            ..Default::default()
        },
        CellContent {
            content: '╕',
            bottom: 1,
            left: 2,
            ..Default::default()
        },
        CellContent {
            content: '╜',
            top: 2,
            left: 1,
            ..Default::default()
        },
        CellContent {
            content: '╛',
            top: 1,
            left: 2,
            ..Default::default()
        },
        CellContent {
            content: '┐',
            bottom: 1,
            left: 1,
            ..Default::default()
        },
        CellContent {
            content: '└',
            top: 1,
            right: 1,
            ..Default::default()
        },
        CellContent {
            content: '┴',
            top: 1,
            right: 1,
            left: 1,
            ..Default::default()
        },
        CellContent {
            content: '┬',
            right: 1,
            bottom: 1,
            left: 1,
            ..Default::default()
        },
        CellContent {
            content: '├',
            top: 1,
            right: 1,
            bottom: 1,
            ..Default::default()
        },
        CellContent {
            content: '─',
            right: 1,
            left: 1,
            ..Default::default()
        },
        CellContent {
            content: '┼',
            top: 1,
            right: 1,
            bottom: 1,
            left: 1,
            ..Default::default()
        },
        CellContent {
            content: '╞',
            top: 1,
            right: 2,
            bottom: 1,
            ..Default::default()
        },
        CellContent {
            content: '╟',
            top: 2,
            right: 1,
            bottom: 2,
            ..Default::default()
        },
        CellContent {
            content: '╧',
            top: 1,
            right: 2,
            left: 2,
            ..Default::default()
        },
        CellContent {
            content: '╨',
            top: 2,
            right: 1,
            left: 1,
            ..Default::default()
        },
        CellContent {
            content: '╤',
            right: 2,
            bottom: 1,
            left: 2,
            ..Default::default()
        },
        CellContent {
            content: '╥',
            right: 1,
            bottom: 2,
            left: 1,
            ..Default::default()
        },
        CellContent {
            content: '╙',
            top: 2,
            right: 1,
            ..Default::default()
        },
        CellContent {
            content: '╘',
            top: 1,
            right: 2,
            ..Default::default()
        },
        CellContent {
            content: '╒',
            right: 2,
            bottom: 1,
            ..Default::default()
        },
        CellContent {
            content: '╓',
            right: 1,
            bottom: 2,
            ..Default::default()
        },
        CellContent {
            content: '╫',
            top: 2,
            right: 1,
            bottom: 2,
            left: 1,
            ..Default::default()
        },
        CellContent {
            content: '╪',
            top: 1,
            right: 2,
            bottom: 1,
            left: 2,
            ..Default::default()
        },
        CellContent {
            content: '┘',
            top: 1,
            left: 1,
            ..Default::default()
        },
        CellContent {
            content: '┌',
            right: 1,
            bottom: 1,
            ..Default::default()
        },
        // CellContent {content: '', ..Default::default()},
    ]
}
