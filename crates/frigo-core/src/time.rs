//! TheMealDB ne donne pas le temps de préparation : on le lit dans les instructions
//! (« bake 35 minutes », « simmer for 1-2 hours ») et on ajoute la préparation.

/// Le temps estimé, en minutes : les durées écrites dans les instructions, plus environ
/// une minute et demie par ingrédient pour le préparer. Sans durée écrite, cinq minutes
/// par étape.
///
/// ```
/// let minutes = frigo_core::estimate_minutes("Fry for 5 minutes.\nBake 1 hour.", 4);
/// assert_eq!(minutes, 5 + 60 + 6);
/// ```
pub fn estimate_minutes(instructions: &str, ingredients: usize) -> u32 {
    let prep = (ingredients as u32 * 3).div_ceil(2);
    let written = written_minutes(instructions);
    let cooking = if written > 0 {
        written
    } else {
        let steps = instructions
            .split(['\n', '.'])
            .filter(|s| s.trim().len() > 12)
            .count()
            .max(1) as u32;
        steps * 5
    };
    (cooking + prep).clamp(5, 24 * 60)
}

/// La somme des durées écrites : « 15-20 minutes » compte 20, « 1 1/2 hours » compte 90.
fn written_minutes(text: &str) -> u32 {
    let lower = text.to_lowercase();
    let words: Vec<&str> = lower
        .split(|c: char| c.is_whitespace() || matches!(c, ',' | ';' | '(' | ')' | ':'))
        .filter(|w| !w.is_empty())
        .collect();
    let mut total = 0.0_f32;
    for (i, word) in words.iter().enumerate() {
        let word = word.trim_end_matches(['.', '!']);
        let unit = if word.starts_with("min") {
            1.0
        } else if word.starts_with("hour") || word == "hr" || word == "hrs" || word == "h" {
            60.0
        } else {
            continue;
        };
        // « 10 minutes », « 10-15 minutes », « 1 1/2 hours », « 10 to 15 minutes »
        let mut fraction = 0.0;
        let mut value = None;
        let mut j = i;
        while j > 0 && i - j < 4 {
            j -= 1;
            let w = words[j];
            match number(w) {
                // « 1 1/2 » : la fraction, puis l'entier qui la précède.
                Some(n) if w.contains('/') || matches!(w, "½" | "¼" | "¾") => fraction = n,
                Some(n) => {
                    value = Some(n);
                    break;
                }
                // « 10 to 15 minutes » : le nombre juste avant l'unité est le plus grand.
                None => break,
            }
        }
        let value = value.unwrap_or(0.0) + fraction;
        if value > 0.0 {
            total += value * unit;
        }
    }
    total.round().min(u32::MAX as f32) as u32
}

/// `"20"`, `"15-20"` (le plus grand), `"1/2"`, `"½"`.
fn number(word: &str) -> Option<f32> {
    if let Some((a, b)) = word.split_once(['-', '–']) {
        return number(b).or_else(|| number(a));
    }
    if let Some((a, b)) = word.split_once('/') {
        let (a, b): (f32, f32) = (a.parse().ok()?, b.parse().ok()?);
        return (b > 0.0).then(|| a / b);
    }
    match word {
        "½" => return Some(0.5),
        "¼" => return Some(0.25),
        "¾" => return Some(0.75),
        "a" | "an" | "one" => return Some(1.0),
        "two" => return Some(2.0),
        "few" => return Some(3.0),
        _ => {}
    }
    word.parse::<f32>()
        .ok()
        .filter(|n| n.is_finite() && *n >= 0.0 && *n < 1000.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranges_take_the_longest() {
        assert_eq!(written_minutes("cook for 15-20 minutes"), 20);
        assert_eq!(written_minutes("cook 10 to 15 mins"), 15);
    }

    #[test]
    fn hours_and_fractions() {
        assert_eq!(written_minutes("simmer 2 hours"), 120);
        assert_eq!(written_minutes("bake 1 1/2 hours"), 90);
        assert_eq!(written_minutes("rest for an hour"), 60);
    }

    #[test]
    fn seconds_do_not_count() {
        assert_eq!(written_minutes("stir for 10 seconds"), 0);
    }
}
