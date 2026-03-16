pub mod node;
pub mod tree;
pub mod list;
pub mod grid;

// Re-eksportujemy dla wygody, aby w engine.rs używać PathTree i FileNode bezpośrednio
pub use tree::PathTree;
pub use list::PathList;
pub use grid::PathGrid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    Tree,
    List,
    Grid,
}