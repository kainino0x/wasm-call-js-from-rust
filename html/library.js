// This file is referenced by src/externs.rs in the #[link_args]

// *ARCANE INCANTATION ALERT*
mergeInto(LibraryManager.library, {
  // - At compile/link time, this hooks up the FFI externs for runtime linking.
  // - At runtime (in the browser), this defines the implementation.
  alert: function(x) {
    window.alert(Pointer_stringify(x));
  },
});
