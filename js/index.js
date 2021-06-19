import "../style/index.scss";
import SolverWorker from "worker-loader!./solver.js";

function test() {
  console.log("testing");
}

import("../pkg").then((lib) => {
  let worker = new SolverWorker();
  let app = new lib.App();
  worker.onmessage = function (e) {
    if (e.data[0] == "init") {
      function solve(sudoku, rules) {
        worker.postMessage([sudoku, rules]);
      }
      app.set_solver(solve);
      app.start();
    } else if (e.data[0] == "solved") {
      console.log(e.data[1]);
      app.on_solve(e.data[1]);
      app.on_measure(e.data[2]);
    }
  };
  worker.postMessage(["init"]);
});
