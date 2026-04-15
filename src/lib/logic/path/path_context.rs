/// ============================================================================
/// PATH CONTEXT (ZERO-COPY / STACK-BASED)
/// ============================================================================
///
/// Reprezentuje semantyczny podział ścieżki na część katalogową (parent)
/// oraz nazwę pliku (file). Działa w 100% na pożyczonych referencjach (zero
/// alokacji na stercie).

#[derive(Debug, Clone)]
pub struct PathContext<'a> {
	pub parent: &'a str,
	pub file: &'a str,
}

impl<'a> PathContext<'a> {
	/// Tworzy kontekst z surowej ścieżki tekstowej bez kopiowania pamięci.
	pub fn from(path: &'a str) -> Self {
		let clean_path = path.trim_start_matches("./");

		// Zamiast tworzyć wektory i łączyć stringi, robimy matematykę na indeksach
		// (super szybkie!)
		let (parent, file) = match clean_path.rfind('/') {
			Some(idx) => {
				// Odcinamy wszystko do ostatniego ukośnika (to jest parent)
				// i wszystko po nim (to jest file)
				(&clean_path[..idx], &clean_path[idx + 1..])
			}
			None => {
				// Brak ukośnika - plik jest w katalogu głównym
				("", clean_path)
			}
		};

		Self { parent, file }
	}

	pub fn name(&self) -> &'a str { self.file }

	pub fn parent(&self) -> &'a str { self.parent }

	pub fn is_root_level(&self) -> bool { self.parent.is_empty() }

	// Zauważ: usunąłem funkcję `full()`, bo wymuszałaby znowu łączenie stringów
	// (alokację). Jeśli silnik DSL jej nie używa, wywalamy ją dla bezpieczeństwa.
}
