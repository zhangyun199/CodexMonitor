pub mod supabase;
pub mod embeddings;
pub mod service;

pub use service::MemoryService;

// Note: auto_flush module is not included here because it depends on
// backend::app_server and types modules which may not be available in all
// compilation contexts (e.g., standalone MCP binaries).
// Consumers that need auto_flush should include it separately with:
//   #[path = "../memory/auto_flush.rs"]
//   mod auto_flush;
