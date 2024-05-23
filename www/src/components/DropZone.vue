<template>
    <div>
        <div v-if="!refProcessed" v-bind='getRootPropsRef()' class="dropzone dropzone-ref">
            <input v-bind='getInputPropsRef()' />
            <p v-if='isDragActiveRef' class="dropzone-text">Drop the files here ...</p>
            <p v-else class="dropzone-text">Drag and drop your <b>reference fasta file</b> here,
                or click to select a file</p>
        </div>

        <div v-if="refProcessed" class="dropzone dropzone-ref">
            <p class="dropzone-text">✅ Reference indexed: <span class="monospace">{{ refName }}</span></p>
        </div>

        <!-- Mapping tab -->
        <div v-if="tabName=='Mapping'">
            <div v-if="refProcessed" v-bind='getRootPropsQueryMap()' class="dropzone dropzone-query">
                <input v-bind='getInputPropsQueryMap()' />
                <p v-if='isDragActiveQueryMap' class="dropzone-text">Drop the files here ...</p>
                <p v-else class="dropzone-text">Drag and drop read or assembly <b>files to be mapped</b> here,
                    or click to select files</p>
            </div>
            <p v-if="refProcessed" class="count"> Files received: {{ Object.keys(allResults.mapResults).length }}</p>
        </div>

        <!-- Alignment tab -->
        <div v-else-if="tabName=='Alignment'">
            <div v-if="refProcessed" v-bind='getRootPropsQueryAlign()' class="dropzone dropzone-query">
                <input v-bind='getInputPropsQueryAlign()' />
                <p v-if='isDragActiveQueryAlign' class="dropzone-text">Drop the files here ...</p>
                <p v-else class="dropzone-text">Drag and drop read or assembly <b>files to be aligned</b> here,
                    or click to select files</p>
            </div>
            <p v-if="refProcessed" class="count"> Files received: {{ Object.keys(allResults.alignResults).length }}</p>
        </div>
    </div>
</template>

<script>
import { useDropzone } from "vue3-dropzone";
import { useActions, useState } from "vuex-composition-helpers";

export default {
    name: "DropZone",
    props:["tabName"],
    setup() {
        const { processRef, processQueryMap, processQueryAlign } = useActions(["processRef", "processQueryMap", "processQueryAlign"]);
        const { allResults } = useState(["allResults"]);

        function onDropRef(acceptFiles) {
            processRef(acceptFiles);
        }
        function onDropQueryMap(acceptFiles) {
            processQueryMap(acceptFiles);
        }
        function onDropQueryAlign(acceptFiles) {
            processQueryAlign(acceptFiles);
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
            getRootProps: getRootPropsQueryMap,
            getInputProps: getInputPropsQueryMap,
            isDragActive: isDragActiveQueryMap,
            ...restQueryMap
        } = useDropzone({
            onDrop: onDropQueryMap,
            accept: [".fa", ".fasta", ".gz", ".fastq", ".fq"]
        });
        const {
            getRootProps: getRootPropsQueryAlign,
            getInputProps: getInputPropsQueryAlign,
            isDragActive: isDragActiveQueryAlign,
            ...restQueryAlign
        } = useDropzone({
            onDrop: onDropQueryAlign,
            accept: [".fa", ".fasta", ".gz", ".fastq", ".fq"] // To be redifined
        });

        return {
            getRootPropsRef,
            getInputPropsRef,
            isDragActiveRef,
            getRootPropsQueryMap,
            getInputPropsQueryMap,
            isDragActiveQueryMap,
            getRootPropsQueryAlign,
            getInputPropsQueryAlign,
            isDragActiveQueryAlign,
            onDropRef,
            onDropQueryMap,
            onDropQueryAlign,
            allResults,
            ...restRef,
            ...restQueryMap,
            ...restQueryAlign
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
    margin-top: 30px;
    background-color: rgb(159, 176, 190);
}

.dropzone-query {
    height: 75px;
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