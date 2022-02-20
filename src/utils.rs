use tower_lsp::lsp_types::{Position, Range};

struct Line {
    offset: usize,
    len: usize,
}

pub fn pos_to_range(text: &String, (start, end): (usize, usize)) -> Range {
    let text = text.replace("\r\n", "\n");
    let mut offset = 0;

    let lines = text
        .lines()
        .map(|line| {
            let l = Line {
                offset,
                len: line.chars().count() + 1, // TODO: Don't assume all newlines are a single character!
            };
            offset += l.len;
            l
        })
        .collect::<Vec<_>>();

    let get_pos = |pos: usize| {
        let idx = lines
            .binary_search_by_key(&pos, |line| line.offset)
            .unwrap_or_else(|idx| idx.saturating_sub(1));
        let line = &lines[idx];
        assert!(
            pos >= line.offset,
            "offset = {}, line.offset = {}",
            pos,
            line.offset
        );
        Position {
            line: idx as u32,
            character: (pos - line.offset) as u32,
        }
    };

    Range {
        start: get_pos(start),
        // TODO: figure out why this is sometimes 1 character off
        end: get_pos(end),
    }
}
