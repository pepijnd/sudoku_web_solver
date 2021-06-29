import "../style/index.scss";
import SolverWorker from "worker-loader!./worker.js";

function test() {
  console.log("testing");
}

import("../pkg_webui").then((lib) => {
  let app = new lib.App();
  app.start(SolverWorker);
});
