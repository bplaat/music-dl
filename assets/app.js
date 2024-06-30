window.app = new Vue({
  el: "#app",
  data: {
    searchQuery: "",
    searchResults: [],
  },
  methods: {
    search() {
      window.ipc.postMessage(`search:${this.searchQuery}`);
    },
    searchResult(albums, artists) {
      document.write(data);
    },
  },
});
