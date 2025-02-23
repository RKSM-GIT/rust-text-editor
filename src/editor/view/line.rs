use std::{fmt, ops::Range};
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

pub struct TextFragment {
    grapheme: String,
    rendered_width: GraphemeWidth,
    replacement: Option<char>,
}

pub struct Line {
    fragments: Vec<TextFragment>,
}

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        let fragments: Vec<TextFragment> = Self::str_to_fragments(value);
        Self { fragments }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let result: String = self
            .fragments
            .iter()
            .map(|fragment| fragment.grapheme.clone())
            .collect();
        write!(formatter, "{result}")
    }
}
impl Line {

    pub fn new(fragments: Vec<TextFragment>) -> Self {
        Self { fragments }
    }
    
    fn str_to_fragments(value: &str) -> Vec<TextFragment> {
        value
            .graphemes(true)
            .map(|grapheme| {
                let (replacement, rendered_width) = Self::replacement_character(grapheme)
                    .map_or(
                        {
                            let unicode_width = grapheme.width();
                            let rendered_width = match unicode_width {
                                0 | 1 => GraphemeWidth::Half,
                                _ => GraphemeWidth::Full,
                            };
                            (None, rendered_width)
                        },
                        |replacement| (Some(replacement), GraphemeWidth::Half),
                    );

                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                }
            })
            .collect()
    } 

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

    pub fn width_until(&self, grapheme_index: usize) -> usize {
        self.fragments
            .iter()
            .take(grapheme_index)
            .map(|fragment| fragment.rendered_width.width())
            .sum()
    }

    fn replacement_character(s: &str) -> Option<char> {
        if s == " " {
            None
        } else if s == "\t" {
            Some(' ')
        } else if s.width() > 0 && s.trim().is_empty() {
            Some('␣')
        } else if s.width() == 0 {
            let mut iterator = s.chars();
            if let Some(ch) = iterator.next() {
                if ch.is_control() && iterator.next().is_none() {
                    return Some('▯');
                } else {
                    return Some(' ');
                }
            }
            Some('.')
        } else {
            None
        }
    }

    pub fn insert_char(&mut self, c: char, grapheme_index: usize) {
        let mut new_str = String::new();
        let mut has_pushed = false;

        for (ind, fragment) in self.fragments.iter().enumerate() {
            if ind == grapheme_index {
                new_str.push(c);
                has_pushed = true;
            }
            new_str.push_str(&fragment.grapheme);
        }

        if !has_pushed {
            new_str.push(c);
        }

        self.fragments = Self::str_to_fragments(&new_str);
    }

    pub fn delete_grapheme_at(&mut self, grapheme_index: usize) {
        let mut new_str = String::new();

        for (ind, fragment) in self.fragments.iter().enumerate() {
            if ind != grapheme_index {
                new_str.push_str(&fragment.grapheme);
            }
        }

        self.fragments = Self::str_to_fragments(&new_str);
    }

    pub fn as_string(&self) -> String {
        let mut res = String::new();

        for TextFragment { grapheme, .. } in self.fragments.iter() {
            res.push_str(grapheme);
        }

        res
    }

    pub fn append_str(&mut self, s: &str) {
        let mut res = self.as_string();
        res.push_str(s);
        self.fragments = Self::str_to_fragments(&res);
    }

    pub fn split(&mut self, grapheme_index: usize) -> Vec<TextFragment> {
        self.fragments.split_off(grapheme_index)
    }    
}
