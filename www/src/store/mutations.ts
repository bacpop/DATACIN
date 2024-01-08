import { RootState } from "@/store/state";

export default {
    SET_WORKER(state: RootState, worker: Worker | null) {
        state.workerState.worker = worker;
    },
    addRef(state: RootState, name: string) {
        console.log("vuex: Adding ref " + name);
        state.refSet = name;
    },
    addQueryFile(state: RootState, name: string) {
        console.log("vuex: Adding query file " + name)
        if (!state.allResults.mapResults[name]) {
            state.allResults.mapResults[name] = {
                mapped: false
            };
        }
    },
    setMapped(state: RootState, input: string) {
        state.allResults.mapResults[input].mapped = true
    },
};
