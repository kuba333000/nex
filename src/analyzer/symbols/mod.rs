mod scope;
mod symbol;
mod table;

pub use self::scope::{Scope, ScopeId};
pub use self::symbol::{CallableKind, CallableObj, Symbol, SymbolId, SymbolKind, TypeId};
pub use self::table::SymbolTable;