# Convertisseur Celsius/Fahrenheit

Mon projet est un convertisseur de température qui permet de convertir des valeurs entre Celsius et Fahrenheit.

Ce projet a été réalisé pour mieux comprendre Rust et apprendre à utiliser sa documentation.

## Principe du projet

Le programme demande à l'utilisateur de choisir entre deux options : convertir une température de Celsius à Fahrenheit ou de Fahrenheit à Celsius. Ensuite, il demande à l'utilisateur d'entrer la valeur de température à convertir et affiche le résultat.

## Prérequis

Pour compiler et exécuter ce projet, vous devez avoir installé :

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo, installé automatiquement avec Rust

Vous pouvez vérifier votre installation avec les commandes suivantes :

```bash
rustc --version
cargo --version
```

## Installation

Clonez le dépôt GitHub :

```bash
git clone https://github.com/goaanthony/convert_temperature_rust.git
```

Accédez ensuite au dossier du projet :

```bash
cd convert_temperature_rust
```

## Lancer le convertisseur

Pour compiler et lancer le programme, utilisez :

```bash
cargo run
```

La commande `cargo run` compile automatiquement le projet avant de l'exécuter.

Vous pouvez également compiler le projet séparément avec :

```bash
cargo build
```

## Structure du projet

```text
convert_temperature_rust/
├── Cargo.toml
├── Cargo.lock
└── src/
    └── main.rs
```

## Licence

Ce projet est disponible à des fins d'apprentissage.