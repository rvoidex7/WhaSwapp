#!/bin/bash

# Apply local patches to submodules
echo "Applying patches..."

# Patch whatsapp-rust
if [ -d "whatsapp-rust" ]; then
    echo "Patching whatsapp-rust..."
    # Check if patch is already applied
    if grep -q "fn register_handler" whatsapp-rust/src/client.rs; then
        echo "Patch already applied."
    else
        cd whatsapp-rust
        # Apply patch with more permissive context matching
        patch -p1 --ignore-whitespace <<EOF
--- a/src/client.rs
+++ b/src/client.rs
@@ -380,6 +380,11 @@
         router
     }

+    /// Registers an external event handler to the core event bus.
+    pub fn register_handler(&self, handler: std::sync::Arc<dyn crate::types::events::EventHandler>) {
+        self.core.event_bus.add_handler(handler);
+    }
+
     pub async fn run(self: &Arc<Self>) {
         if self.is_running.swap(true, Ordering::SeqCst) {
             warn!("Client \`run\` method called while already running.");
EOF
        cd ..
    fi
fi

echo "Patches applied."
