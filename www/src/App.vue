<template>
    <div id="app">
        <img src="datacin.png" alt="Datacin Logo" class="app-logo">
        <DropZone />
        <ResultsDisplay />
    </div>
</template>

<script>
import DropZone from './components/DropZone.vue';
import ResultsDisplay from './components/ResultsDisplay.vue';
import WorkerMapper from '@/workers/Mapper.worker.js';

export default {
    name: 'App',
    components: {
        DropZone,
        ResultsDisplay
    },
    mounted: function () {
        console.log("Loading wasm module in a worker thread")
        return import("@/pkg")
            .then(() => {
                if (window.Worker) {
                    const worker = new WorkerMapper();
                    this.$store.commit('SET_WORKER', worker);
                } else {
                    throw "WebWorkers are not supported by this web browser.";
                }
            });
    },
}

</script>

<style>
#app {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-align: center;
    color: #2c3e50;
    margin-top: 60px;
}

.app-logo {
    position: absolute;
    top: 10px;
    left: 10px;
    height: 75px;
}
</style>