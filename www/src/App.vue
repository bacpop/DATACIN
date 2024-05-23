<template>
    <div id="app">
        <div id="Header">
            <img src="datacin.png" alt="Datacin Logo" class="app-logo">
            <div id="tabs">
                <button class="tab" v-on:click="changeTab('Mapping')">Mapping</button>
                <button class="tab" v-on:click="changeTab('Alignment')">Alignment</button>
            </div>
        </div>
        <DropZone 
            :tabName="tabName"
        />
        <div v-if="tabName === 'Mapping'">
            <ResultsDisplayMapping />
        </div>
    </div>
</template>

<script>
import DropZone from './components/DropZone.vue';
import ResultsDisplayMapping from './components/ResultsDisplayMapping.vue';
import WorkerMapper from '@/workers/Mapper.worker.js';

export default {
    name: 'App',
    components: {
        DropZone,
        ResultsDisplayMapping
    },
    data() {
        return {
            tabName: 'Mapping'
        }
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

    methods: {
        changeTab: function (tab) {
            this.tabName = tab;
        }
    }
}

</script>

<style>
#app {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-align: center;
    color: #2c3e50;
}

.app-logo {
    float: left;
    height: 75px;
}

#Header {
    height: 75px;
    margin-top: 0px;
}

#tabs {
    float: left;
    margin-left: 30px;
    height: 100%;
    display: flex;
    align-items: center;
}

.tab {
    background-color: #f2f2f2;
    text-align: center;
    font-size: 16px;
    cursor: pointer;
    padding: 14px 16px;
    border: 1px solid black;
    display: inline-block;
    margin-left: 10px;
}

.tab:hover {
    background-color: #ddd;
}

</style>