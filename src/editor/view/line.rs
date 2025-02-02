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
    width_prefix_sum: Vec<usize>,
}

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        let fragments: Vec<TextFragment> = value
            .graphemes(true)
            .map(|grapheme| {
                let unicode_width = grapheme.width();
                let rendered_width = match unicode_width {
                    0 | 1 => GraphemeWidth::Half,
                    _ => GraphemeWidth::Full,
                };
                let replacement = match unicode_width {
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

        let mut width_prefix_sum = vec![0; fragments.len()];

        for i in 1..fragments.len() {
            width_prefix_sum[i] = width_prefix_sum[i - 1] + fragments[i - 1].rendered_width.width();
        }

        Self {
            fragments,
            width_prefix_sum,
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

    pub fn width_until(&self, grapheme_ind: usize) -> usize {
        self.width_prefix_sum
            .get(grapheme_ind)
            .map_or(0, |width| *width)
    }
}
