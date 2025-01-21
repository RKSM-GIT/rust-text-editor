use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

enum GraphemeWidth {
    Half,
    Full,
}

impl GraphemeWidth {
    const fn width(&self) -> usize {
        match self {
            Self::Half => 1,
            Self::Full => 2,
        }
    }
}

struct TextFragment {
    grapheme: String,
    rendered_width: GraphemeWidth,
    replacement: Option<char>,
}

pub struct Line {
    fragments: Vec<TextFragment>,
    total_width: usize,
}

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        let mut total_width = 0;
        let fragments = value
            .graphemes(true)
            .map(|grapheme| {
                let rendered_width = match grapheme.width() {
                    0 | 1 => GraphemeWidth::Half,
                    _ => GraphemeWidth::Full,
                };

                total_width += rendered_width.width();

                let replacement = match grapheme.width() {
                    0 => Some('.'),
                    _ => None,
                };

                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                }
            })
            .collect();

        Self {
            fragments,
            total_width,
        }
    }
}

impl Line {
    pub fn get(&self, range: Range<usize>) -> String {
        let mut res = String::new();
        let (l, r) = (range.start, range.end);
        let mut pos_ind = 0;

        if l >= r {
            return res;
        }

        for grapheme_ind in 0..self.grapheme_count() {
            if pos_ind >= r {
                break;
            }

            let width = self.fragments[grapheme_ind].rendered_width.width();

            if width == 2 && (l == pos_ind + 1 || r == pos_ind + 1) {
                res.push('⋯');
            } else if pos_ind >= l {
                match self.fragments[grapheme_ind].replacement {
                    Some(s) => res.push(s),
                    None => res.push_str(&self.fragments[grapheme_ind].grapheme),
                }
            }

            pos_ind += width;
        }

        res
    }

    pub fn grapheme_count(&self) -> usize {
        self.fragments.len()
    }

    pub fn total_width(&self) -> usize {
        self.total_width
    }

    pub fn width_until(&self, grapheme_ind: usize) -> usize {
        self.fragments
            .iter()
            .take(grapheme_ind)
            .map(|x| x.rendered_width.width())
            .sum()
    }
}
