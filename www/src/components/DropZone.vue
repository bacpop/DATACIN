<template>
    <div>
        <div v-if="!refProcessed" v-bind='getRootPropsRef()' class="dropzone dropzone-ref">
            <input v-bind='getInputPropsRef()' />
            <p v-if='isDragActiveRef' class="dropzone-text">Drop the files here ...</p>
            <p v-else class="dropzone-text">Drag and drop your <b>reference fasta file</b> here,
                or click to select a file</p>
        </div>
        <!-- dummy window after ref processed-->
        <div v-if="refProcessed" class="dropzone dropzone-ref">
            <p class="dropzone-text">✅ Reference indexed: <span class="monospace">{{ refName }}</span></p>
        </div>
        <div v-if="refProcessed" v-bind='getRootPropsQuery()' class="dropzone dropzone-query">
            <input v-bind='getInputPropsQuery()' />
            <p v-if='isDragActiveQuery' class="dropzone-text">Drop the files here ...</p>
            <p v-else class="dropzone-text">Drag and drop read or assembly <b>files to be mapped</b> here,
                or click to select files</p>
        </div>
        <p v-if="refProcessed" class="count"> Files received: {{ Object.keys(allResults.mapResults).length }}</p>
    </div>
</template>

<script>
import { useDropzone } from "vue3-dropzone";
import { useActions, useState } from "vuex-composition-helpers";

export default {
    name: "DropZone",
    setup() {
        const { processRef, processQuery } = useActions(["processRef", "processQuery"]);
        const { allResults } = useState(["allResults"]);

        function onDropRef(acceptFiles) {
            processRef(acceptFiles);
        }
        function onDropQuery(acceptFiles) {
            processQuery(acceptFiles);
        }
        const {
            getRootProps: getRootPropsRef,
            getInputProps: getInputPropsRef,
            isDragActive: isDragActiveRef,
            ...restRef
        } = useDropzone({
            onDrop: onDropRef,
            accept: [".fa", ".fasta"],
            multiple: false
        });
        const {
            getRootProps: getRootPropsQuery,
            getInputProps: getInputPropsQuery,
            isDragActive: isDragActiveQuery,
            ...restQuery
        } = useDropzone({
            onDrop: onDropQuery,
            accept: [".fa", ".fasta", ".gz", ".fastq", ".fq"]
        });

        return {
            getRootPropsRef,
            getInputPropsRef,
            isDragActiveRef,
            getRootPropsQuery,
            getInputPropsQuery,
            isDragActiveQuery,
            onDropRef,
            onDropQuery,
            allResults,
            ...restRef,
            ...restQuery
        };
    },
    computed: {
        refProcessed() {
            return this.$store.getters.refProcessed;
        },
        refName() {
            return this.$store.getters.refName;
        }
    }
};
</script>

<style>
.dropzone {
    border: 2px dotted rgb(56, 55, 55);
    width: 80%;
    margin: 10%;
    text-align: center;
    vertical-align: middle;
    display: flex;
    align-items: center;
    border-radius: 4px;
    margin-bottom: 5px;
}

.dropzone-ref {
    height: 75px;
    margin-top: 100px;
    background-color: rgb(159, 176, 190);
}

.dropzone-query {
    height: 200px;
    margin-top: 10px;
    background-color: rgb(221, 249, 226);
}

.dropzone-text {
    padding: 30px;
}

.monospace {
    font-family: 'Courier New', monospace;
}
</style>