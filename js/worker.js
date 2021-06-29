import("../pkg_worker").then((lib) => {
  let worker = new lib.Worker(self);
});