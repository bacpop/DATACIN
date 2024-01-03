if ('function' === typeof importScripts) {
    importScripts('./web_map.js');
  }

  const workdir = '/working';

  function createFS(module, f) {
    // create working directory in filesystem
    module.FS.mkdir(workdir);
    // mount file to work dir
    module.FS.mount(module.FS.filesystems.WORKERFS, { files: [f] }, workdir);
  }

  onmessage = function (message) {
    // run mapping
    ska_map().then(module => {
      const f = message.data.fileObject;
      //create working directory and mount file
      createFS(module, f);
      // run mapping
      // TODO: we will need to take a reference genome
      const ska_result = module.fastmap(workdir + "/" + f.name);
      // return result
      postMessage({ filename: message.data.filename, mapping: ska_result });
    });

  };
