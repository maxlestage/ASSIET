//! `frigo` : que cuisiner avec ce qu'il reste dans le frigo ?
//!
//! ```text
//! frigo poulet riz oignon            # la recette la plus rapide, dans la console
//! frigo -f frigo.txt --strict        # seulement ce qui est faisable sans courses
//! frigo --tui                        # l'interface plein écran
//! echo "oeufs, lait" | frigo --json  # pour les scripts
//! ```

mod cli;
mod print;
mod tui;

use std::io::{IsTerminal, Read};

use clap::Parser;
use color_eyre::eyre::{Result, WrapErr, bail};
use frigo_core::{MealDb, parse_list};

use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let fridge = read_fridge(&cli)?;
    let mut db = MealDb::new(&cli.api);
    db.max_details = cli.details;

    if cli.tui || (fridge.is_empty() && std::io::stdout().is_terminal()) {
        return tui::run(db, fridge, cli.criteria()).await;
    }
    if fridge.is_empty() {
        bail!("le frigo est vide : `frigo poulet riz`, `frigo -f frigo.txt` ou `frigo --tui`");
    }

    let matches = db.suggest(&fridge, &cli.criteria()).await?;
    let shown: Vec<_> = matches.into_iter().take(cli.count).collect();
    if cli.json {
        println!("{}", serde_json::to_string_pretty(&shown)?);
    } else {
        print::report(
            &fridge,
            &shown,
            cli.no_color || !std::io::stdout().is_terminal(),
        );
    }
    if shown.is_empty() {
        std::process::exit(1);
    }
    Ok(())
}

/// Les ingrédients des arguments, du fichier (`-` : l'entrée standard), de l'entrée
/// standard quand elle n'est pas un terminal, ou de `frigo.txt` s'il existe.
fn read_fridge(cli: &Cli) -> Result<Vec<String>> {
    let mut text = cli.ingredients.join(",");
    match cli.file.as_deref() {
        Some("-") => {
            text.push('\n');
            std::io::stdin().read_to_string(&mut text)?;
        }
        Some(path) => {
            text.push('\n');
            text.push_str(
                &std::fs::read_to_string(path).wrap_err_with(|| format!("lecture de {path}"))?,
            );
        }
        None if cli.ingredients.is_empty() && !std::io::stdin().is_terminal() && !cli.tui => {
            std::io::stdin().read_to_string(&mut text)?;
        }
        None if cli.ingredients.is_empty() => {
            if let Ok(saved) = std::fs::read_to_string("frigo.txt") {
                text.push_str(&saved);
            }
        }
        None => {}
    }
    Ok(parse_list(&text))
}
