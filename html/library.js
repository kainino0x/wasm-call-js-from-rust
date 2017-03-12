mergeInto(LibraryManager.library, {
  alert: function(x) {
    window.alert(Pointer_stringify(x));
  },
});
