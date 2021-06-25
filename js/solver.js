onmessage = function (e) {
  if (e.data[0] == "init") {
    import("../pkg_solver").then((lib) => {
      onmessage = function (e) {
        performance.mark("perf_start");
        let solve = lib.solve(e.data[0], e.data[1], function(progress) {
          postMessage(["progress", progress]);
        });
        performance.mark("perf_stop");
        performance.measure("perf_measure", "perf_start", "perf_stop");
        let entries = performance.getEntriesByName("perf_measure");
        let measure = entries[entries.length - 1];
        postMessage([
          "solved",
          solve,
          {
            name: measure.name,
            startTime: measure.startTime,
            duration: measure.duration,
          },
        ]);
      };
      postMessage(["init"]);
    });
  }
};
