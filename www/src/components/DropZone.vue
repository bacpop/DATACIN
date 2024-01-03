<template>
    <div>
      <div v-bind='getRootProps()' class=dropzone>
        <input v-bind='getInputProps()' />
        <p v-if='isDragActive' class="dropzone-text">Drop the files here ...</p>
        <p v-else class="dropzone-text">Drag and drop your fasta files here,
          or click to select files</p>
      </div>
      <p class="count"> Uploaded files: {{ Object.keys(allResults.mapResults).length }}</p>
    </div>
  </template>

  <script>
  import { useDropzone } from "vue3-dropzone";
  import { useActions, useState } from "vuex-composition-helpers";

  export default {
      name: "DropZone",
      setup() {
          const { processFiles } = useActions(["processFiles"]);
          const { allResults } = useState(["allResults"]);
          function onDrop(acceptFiles) {
              processFiles(acceptFiles);
          }
          const {
              getRootProps, getInputProps, isDragActive, ...rest
          } = useDropzone({ onDrop, accept: [".fa", ".fasta", ".fa.gz", ".fasta.gz"] });
          return {
              getRootProps,
              getInputProps,
              isDragActive,
              onDrop,
              allResults,
              ...rest
          };
      }
  };
  </script>

<style>
.dropzone {
    border: 2px dotted rgb(56, 55, 55);
    width: 80%;
    height: 300px;
    border-radius: 4px;
    background-color: rgb(159, 176, 190);
    margin: 10%;
    text-align: center;
    vertical-align: middle;
    display: flex;
    align-items: center;
}

.dropzone-text {
    padding: 30px;
}
</style>