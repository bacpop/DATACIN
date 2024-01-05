import { RootState } from "@/store/state";

export default {
    SET_WORKER(state: RootState, worker: Worker | null) {
        state.workerState.worker = worker;
    },
    set_worker_result(state: RootState, result: string) {
        state.workerState.workerResult = result;
    },
    addRef(state: RootState, name: string) {
        console.log("vuex: Adding ref " + name);
        state.refSet = name;
    },
    addQueryFile(state: RootState, name: string) {
        console.log("vuex: Adding file " + name)
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
