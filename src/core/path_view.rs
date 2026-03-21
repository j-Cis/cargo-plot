pub mod grid;
pub mod list;
pub mod node;
pub mod tree;

// Re-eksportujemy dla wygody, aby w engine.rs używać PathTree i FileNode bezpośrednio
pub use grid::PathGrid;
pub use list::PathList;
pub use tree::PathTree;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    Tree,
    List,
    Grid,
}
