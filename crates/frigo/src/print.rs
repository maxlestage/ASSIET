//! L'affichage dans la console : la meilleure recette en détail, puis les suivantes.

use frigo_core::Match;
use ratatui::crossterm::style::Stylize;

pub fn report(fridge: &[String], matches: &[Match], plain: bool) {
    let paint = |text: String, f: fn(String) -> String| if plain { text } else { f(text) };
    let bold = |t: String| t.bold().to_string();
    let green = |t: String| t.green().to_string();
    let red = |t: String| t.red().to_string();
    let dim = |t: String| t.dark_grey().to_string();
    let accent = |t: String| t.cyan().bold().to_string();

    println!("{} {}", paint("🧊 Frigo :".into(), bold), fridge.join(", "));
    let Some(best) = matches.first() else {
        println!(
            "{}",
            paint(
                "Aucune recette trouvée. Essayez sans --strict, ou avec d'autres ingrédients."
                    .into(),
                red
            )
        );
        return;
    };

    println!();
    println!(
        "{} {}",
        paint("🍽  ".into(), bold),
        paint(best.meal.name.clone(), accent)
    );
    let mut meta = vec![format!("≈ {} min", best.minutes)];
    meta.extend(best.meal.category.clone());
    meta.extend(best.meal.area.clone());
    meta.push(format!("{:.0} % des ingrédients", best.coverage() * 100.0));
    println!("   {}", paint(meta.join(" · "), dim));
    println!();
    for ingredient in &best.meal.ingredients {
        let ok = best.have.contains(&ingredient.name);
        let mark = if ok {
            paint("✓".into(), green)
        } else {
            paint("✗".into(), red)
        };
        let measure = if ingredient.measure.is_empty() {
            String::new()
        } else {
            paint(format!(" ({})", ingredient.measure), dim)
        };
        println!("   {mark} {}{measure}", ingredient.name);
    }
    if best.complete() {
        println!(
            "\n   {}",
            paint("Tout est là : à vos casseroles !".into(), green)
        );
    } else {
        println!(
            "\n   {} {}",
            paint("À acheter :".into(), red),
            best.missing.join(", ")
        );
    }
    println!();
    for (i, step) in steps(&best.meal.instructions).iter().enumerate() {
        println!("   {} {step}", paint(format!("{:>2}.", i + 1), accent));
    }
    if let Some(video) = &best.meal.youtube {
        println!("\n   {} {video}", paint("▶".into(), red));
    }

    if matches.len() > 1 {
        println!("\n{}", paint("Aussi possible :".into(), bold));
        for m in &matches[1..] {
            let missing = if m.complete() {
                paint("rien ne manque".into(), green)
            } else {
                paint(format!("manque {}", m.missing.len()), red)
            };
            println!(
                "   • {} {} · {missing}",
                m.meal.name,
                paint(format!("≈ {} min", m.minutes), dim)
            );
        }
    }
}

/// Les étapes : une par ligne non vide, sans les « STEP 1 » que certaines recettes ont.
pub fn steps(instructions: &str) -> Vec<String> {
    instructions
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .filter(|l| {
            let lower = l.to_lowercase();
            !(lower.starts_with("step") && lower.len() <= 8)
                && !l.chars().all(|c| c.is_ascii_digit() || c == '.')
        })
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn steps_skip_headers() {
        let s = super::steps("STEP 1\nChop.\n\n2.\nCook.\n");
        assert_eq!(s, ["Chop.", "Cook."]);
    }
}
