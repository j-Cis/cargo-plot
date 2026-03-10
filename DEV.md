# 🛠 Notatki Dewelopera (Cheat Sheet)

Ten plik zawiera zbiór najważniejszych komend i procedur używanych podczas tworzenia, utrzymania i publikacji paczki `cargo-plot`. 

## 🧹 Higiena kodu (Zanim zrobisz commit)

Jeśli GitHub Actions (CI) odrzuca Twój kod z powodu błędów formatowania, zawsze przepuść go przez te dwa narzędzia przed wysłaniem na serwer:

* `cargo fmt` – Automatycznie formatuje cały kod do oficjalnego standardu Rusta (naprawia błędy wyrzucane przez CI).
* `cargo clippy` – Uruchamia zaawansowanego lintera, który wyłapuje nieoptymalny lub niebezpieczny kod.

```bash
# Szybka ścieżka naprawcza po odrzuceniu przez CI:
cargo fmt
git add .
git commit -m "style: apply cargo fmt to fix CI pipeline"
git push origin main

```

## 🏷️ Zarządzanie Wydaniami i CI/CD (GitHub Actions)

Proces automatycznego budowania binarek (`.exe`, `.tar.gz`) opiera się na tagach.

### Standardowe wydanie nowej wersji:

```bash
git tag v0.1.2
git push origin v0.1.2

```

### 🚨 Procedura Ratunkowa: Jak cofnąć i naprawić zepsuty Tag?

Jeśli nadałeś tag (np. `v0.1.1`), ale akcja na GitHubie zakończyła się błędem (np. zapomniałeś zrobić `cargo fmt` lub wkradł się błąd), musisz usunąć ten znacznik z obu miejsc i wypchnąć go ponownie po naprawieniu kodu:

```bash
# 1. Usuń zepsuty tag z serwera GitHuba
git push origin --delete v0.1.1

# 2. Usuń zepsuty tag ze swojego lokalnego komputera
git tag -d v0.1.1

# 3. [TUTAJ NAPRAW BŁĄD, ZRÓB COMMIT I PUSH DO MAIN]

# 4. Stwórz nowy tag (już na poprawionym kodzie)
git tag v0.1.1

# 5. Wypchnij go ponownie, by odpalić maszynę budującą na czysto!
git push origin v0.1.1

```

## 📦 Publikacja w rejestrze (Crates.io)

Projekt wykorzystuje **Trusted Publishing** (OIDC). Oznacza to, że po pierwszej ręcznej publikacji, serwer crates.io nie przyjmuje już publikacji bezpośrednio z terminala (API tokenów), lecz ufa wyłącznie plikowi `release.yml` z GitHuba.

Mimo to, komendy lokalne są przydatne do testowania:

* `cargo login` – (Używane tylko raz przy autoryzacji środowiska).
* `cargo publish --dry-run` – Pakuje projekt w izolowanym środowisku i weryfikuje poprawność metadanych w `Cargo.toml`. Zawsze wykonuj przed planowanym wydaniem!

```bash
# Test przed-wydawniczy:
cargo publish --dry-run

```

## ⚡

* `cargo run -- plot tree -s files-first --no-default-excludes -e "./f.md" -e "./d.md" -e "./target/" -e "./.git/" -e "./test/" -e "./.gitignore" -e "./DEV.md" -e "./Cargo.lock" -e "./LICENSE-APACHE" -e "./LICENSE-MIT" -e "./.github/" -e "./.cargo/" -e "./doc/" -e "./README.md" -w binary --weight-precision 5 --no-dir-weight --out-file "f.md" --print-console  --watermark last --print-command --title-file "Struktura Projektu"`

* `cargo plot tree -s files-first --no-default-excludes -e "./f.md" -e "./d.md" -e "./target/" -e "./.git/" -e "./test/" -e "./.gitignore" -e "./DEV.md" -e "./Cargo.lock" -e "./LICENSE-APACHE" -e "./LICENSE-MIT" -e "./.github/" -e "./.cargo/" -e "./doc/" -e "./README.md" -w binary --weight-precision 5 --no-dir-weight --out-file "f.md" --print-console  --watermark last --print-command --title-file "Struktura Projektu"`

* `cargo run -- plot doc --out-dir "." --out "d" -I num -T files-first --no-default-excludes -e "./f.md" -e "./d.md" -e "./target/" -e "./.git/" -e "./test/" -e "./.gitignore" -e "./DEV.md" -e "./Cargo.lock" -e "./LICENSE-APACHE" -e "./LICENSE-MIT" -e "./.github/" -e "./.cargo/" -e "./doc/" -e "./README.md" -w binary --weight-precision 5 --no-dir-weight  --watermark last --print-command --title-file "Dokumentacja Projektu"`

* `cargo plot doc --out-dir "." --out "d" -I num -T files-first --no-default-excludes -e "./f.md" -e "./d.md" -e "./target/" -e "./.git/" -e "./test/" -e "./.gitignore" -e "./DEV.md" -e "./Cargo.lock" -e "./LICENSE-APACHE" -e "./LICENSE-MIT" -e "./.github/" -e "./.cargo/" -e "./doc/" -e "./README.md" -w binary --weight-precision 5 --no-dir-weight  --watermark last --print-command --title-file "Dokumentacja Projektu"`

-------------------------------------------
