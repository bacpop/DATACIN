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
                mapped: false,
                nb_variants: null
            };
        }
    },
    setMapped(state: RootState, input: [string, number|null]) {
        state.allResults.mapResults[input[0]].mapped = true
        state.allResults.mapResults[input[0]].nb_variants = input[1]
    },
};
