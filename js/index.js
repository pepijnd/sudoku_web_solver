import "../style/index.scss";
import SolverWorker from "worker-loader!./solver.js";

function test() {
  console.log("testing");
}

import("../pkg").then((lib) => {
  let worker = new SolverWorker();
  worker.onmessage = function (e) {
    if (e.data[0] == "init") {
      function solve(sudoku) {
        worker.postMessage([sudoku]);
      }
      lib.init();
      lib.set_solver(solve);
      lib.start();
    } else if (e.data[0] == "solved") {
      console.log(e.data[1]);
      lib.on_solve(e.data[1]);
      lib.on_measure(e.data[2]);
    }
  };
  worker.postMessage(["init"]);
});
