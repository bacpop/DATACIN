<template>
    <div>
        <button @click="setRef">Index ref</button>
        <button @click="mapSample">Map a sample</button>
        <p>Worker Response: {{ workerResponse }}</p>
    </div>
    <div id="app">
        <DropZone />
    </div>
</template>

<script>
import DropZone from './components/DropZone.vue';
import WorkerMapper from '@/workers/Mapper.worker.js';

export default {
    name: 'App',
    components: {
        DropZone
    },
    computed: {
        workerResponse() {
            return this.$store.state.workerResult;
        }
    },
    mounted: function () {
        console.log("Loading wasm module")
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
        setRef() {
            this.$store.dispatch('sendMessageToWorker', {ref: true, filename: "ref.fa"});
        },
        mapSample() {
            this.$store.dispatch('sendMessageToWorker', {map: true, filename: "query.fa"});
        }
    }
}

/*
export default {
    name: 'App',
    components: {
        DropZone
    },
    mounted: function () {
        try {
            this.init_wasm()
                .then(() => { this.loop(); })
                .catch((e) => { this.on_error("Error in mounted::promise", e); });
        } catch (e) {
            this.on_error("Error in mounted", e);
        }
    },
    methods: {
        init_wasm: function () {
            try {
                console.log("Loading wasm module")
                return import("@/pkg")
                    .then(() => {
                        this.worker_init();
                    });
            } catch (e) {
                this.on_error("Error in App::init_wasm", e);
            }
        },

        worker_init: function () {
            try {
                if (window.Worker) {
                    this.worker = new WorkerMapper();
                    this.worker.onerror = (evt) => {
                        this.on_error("Error in worker", evt);
                    };
                } else {
                    throw "WebWorkers are not supported by this web browser.";
                }
            } catch (e) {
                this.on_error("Error in App::worker_init", e);
            }
        }
    }
};
*/
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
</style>