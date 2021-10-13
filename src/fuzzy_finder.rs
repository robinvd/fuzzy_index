use std::collections::HashMap;

use inverted_index::{Location, Pos, ReverseIndex};

#[non_exhaustive]
#[derive(Default)]
pub struct FuzzyConfig {
    pub line_span: Option<usize>,
}

/// Score an item
///
/// item has to have len() > 0
fn score_item(config: &FuzzyConfig, item: &[Pos]) -> usize {
    item.windows(2)
        .map(|window| {
            if window[1].line - window[0].line > config.line_span.unwrap_or(usize::MAX) {
                return 1000;
            }

            window[1].offset - window[0].offset
        })
        .sum()
}

pub fn fuzzy_find<'a>(
    config: &FuzzyConfig,
    index: &'a mut ReverseIndex,
    mut query: impl Iterator<Item = &'a str>,
) -> impl Iterator<Item = Location<'a>> {
    let mut results: HashMap<_, Vec<Vec<Pos>>> = HashMap::new();

    if let Some(first) = query.next() {
        for loc in index.query(first) {
            results
                .entry(loc.file.clone())
                .or_default()
                .push(vec![loc.pos])
        }

        for (query_count, query_item) in query.enumerate() {
            let partial_results = index.query(query_item);
            for partial_result in partial_results {
                for entry_results in results.get_mut(&partial_result.file).into_iter().flatten() {
                    if partial_result.pos.offset <= entry_results.last().unwrap().offset
                        || entry_results.len() == query_count + 2
                    {
                        continue;
                    }
                    entry_results.push(partial_result.pos);
                }
            }

            for file_result in results.values_mut() {
                file_result.retain(|vectors| vectors.len() == query_count + 2)
            }
        }
    }

    let mut results_vec: Vec<(usize, &str, Vec<Pos>)> = results
        .into_iter()
        .map(|(file, pos_vec)| pos_vec.into_iter().map(move |pos| (file, pos)))
        .flatten()
        .map(|(file, item)| (score_item(config, &item), file, item))
        .collect();
    results_vec.sort_by_key(|(score, _, _)| *score);
    results_vec
        .into_iter()
        .filter(|(score, _, _)| *score < 1000)
        .map(|(_score, file, vectors)| Location {
            file,
            pos: vectors[0],
        })
}
