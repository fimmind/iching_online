use std::fmt::Display;
use std::ops;

/// I Ching's hexagram's line type
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LineType {
    Yang, // ======
    Yin,  // ==  ==
}

impl ops::Not for LineType {
    type Output = LineType;

    fn not(self) -> Self::Output {
        match self {
            LineType::Yang => LineType::Yin,
            LineType::Yin => LineType::Yang,
        }
    }
}

/// I Ching's hexagram consiting of six lines represented by [`LineType`]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hexagram {
    lines: Vec<LineType>,
}

impl Hexagram {
    /// Create a new hexagram with all the lines set to `LineType::Yang`.
    pub fn new() -> Self {
        Self {
            lines: vec![LineType::Yang; 6],
        }
    }

    /// Build hexagram from it's lines
    ///
    /// # Panics
    /// Panics if lines.len() != 6
    pub fn from_lines(lines: Vec<LineType>) -> Self {
        debug_assert_eq!(lines.len(), 6);
        Self { lines }
    }

    /// Iterator over lines from top to bottm.
    pub fn lines(&self) -> impl DoubleEndedIterator<Item = LineType> + '_ {
        self.lines.iter().copied()
    }

    /// Set type of the given line. `true` is returned if it's value actually,
    /// otherwise `false`.
    pub fn set_line(&mut self, i: usize, line_type: LineType) -> bool {
        if self.lines[i] != line_type {
            self.lines[i] = line_type;
            return true;
        }
        false
    }

    /// Switch type of the given line.
    pub fn switch_line(&mut self, i: usize) {
        self.lines[i] = !self.lines[i]
    }

    /// Get binary id of the inner hexagram.
    ///
    /// Lines are read from top to bottom, yang lines (non-dashed) are treated as
    /// 1s, yin lines (dashed) are treated as 0s. For example `䷁` corresponds to
    /// 0x000000 which is 0, `䷟` corresponds to 0x001110 which is 14, `䷀`
    /// coresponds to 0x111111 which is 63, and so on.
    fn bin_id(&self) -> usize {
        self.lines
            .iter()
            .map(|t| match t {
                LineType::Yang => 1,
                LineType::Yin => 0,
            })
            .fold(0, |id, d| id * 2 + d)
    }

    /// Unicode character for the hexagram.
    pub fn as_char(&self) -> char {
        SIGNS_CHRS[self.bin_id()].0
    }

    /// Traditional id of the hexagram.
    pub fn id(&self) -> usize {
        SIGNS_CHRS[self.bin_id()].1
    }

    /// Chinese name of the hexagram.
    pub fn name(&self) -> &'static str {
        SIGNS_CHRS[self.bin_id()].2
    }
}

impl Display for Hexagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_char().fmt(f)
    }
}

impl Default for Hexagram {
    fn default() -> Self {
        Self::new()
    }
}

/// List of hexagram chars, their ids and names in the binary order
static SIGNS_CHRS: &[(char, usize, &str); 64] = &[
    ('䷁', 2, "乾"),
    ('䷖', 23, "坤"),
    ('䷇', 8, "屯"),
    ('䷓', 20, "蒙"),
    ('䷏', 16, "需"),
    ('䷢', 35, "訟"),
    ('䷬', 45, "師"),
    ('䷋', 12, "比"),
    ('䷎', 15, "小畜"),
    ('䷳', 52, "履"),
    ('䷦', 39, "泰"),
    ('䷴', 53, "否"),
    ('䷽', 62, "同人"),
    ('䷷', 56, "大有"),
    ('䷞', 31, "謙"),
    ('䷠', 33, "豫"),
    ('䷆', 7, "隨"),
    ('䷃', 4, "蠱"),
    ('䷜', 29, "臨"),
    ('䷺', 59, "觀"),
    ('䷧', 40, "噬嗑"),
    ('䷿', 64, "賁"),
    ('䷮', 47, "剝"),
    ('䷅', 6, "復"),
    ('䷭', 46, "無妄"),
    ('䷑', 18, "大畜"),
    ('䷯', 48, "頤"),
    ('䷸', 57, "大過"),
    ('䷟', 32, "坎"),
    ('䷱', 50, "離"),
    ('䷛', 28, "咸"),
    ('䷫', 44, "恆"),
    ('䷗', 24, "遯"),
    ('䷚', 27, "大壯"),
    ('䷂', 3, "晉"),
    ('䷩', 42, "明夷"),
    ('䷲', 51, "家人"),
    ('䷔', 21, "睽"),
    ('䷐', 17, "蹇"),
    ('䷘', 25, "解"),
    ('䷣', 36, "損"),
    ('䷕', 22, "益"),
    ('䷾', 63, "夬"),
    ('䷤', 37, "姤"),
    ('䷶', 55, "萃"),
    ('䷝', 30, "升"),
    ('䷰', 49, "困"),
    ('䷌', 13, "井"),
    ('䷒', 19, "革"),
    ('䷨', 41, "鼎"),
    ('䷻', 60, "震"),
    ('䷼', 61, "艮"),
    ('䷵', 54, "漸"),
    ('䷥', 38, "歸妹"),
    ('䷹', 58, "豐"),
    ('䷉', 10, "旅"),
    ('䷊', 11, "巽"),
    ('䷙', 26, "兌"),
    ('䷄', 5, "渙"),
    ('䷈', 9, "節"),
    ('䷡', 34, "中孚"),
    ('䷍', 14, "小過"),
    ('䷪', 43, "既濟"),
    ('䷀', 1, "未濟"),
];
