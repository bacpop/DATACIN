import { RootState } from "@/store/state";

export default {
    SET_WORKER(state: RootState, worker: Worker | null) {
        state.workerState.worker = worker;
    },
    addRef(state: RootState, input: { name: string, sequences: string[] }) {
        console.log("vuex: Adding ref " + input.name);
        state.refSet = input.name;
        state.allResults.ref = input.sequences;
    },

    addQueryFileMap(state: RootState, name: string) {
        console.log("vuex: Adding query file for mapping " + name)
        if (!state.allResults.mapResults[name]) {
            state.allResults.mapResults[name] = {
                mapped: true,
                nb_variants: null,
                coverage: null,
                mapped_sequences: [],
            };
        }
    },

    addQueryFileAlign(state: RootState, name: string) {
        console.log("vuex: Adding query file for alignment " + name)
        if (!state.allResults.alignResults[name]) {
            state.allResults.alignResults[name] = {
                aligned: true,
                alignment: "",
            };
        }
    },
    
    setMapped(state: RootState, 
              input: {name:string, nb_variants:number|null, coverage:number|null, mapped_sequences:string[]}) {
        state.allResults.mapResults[input.name].nb_variants = input.nb_variants
        state.allResults.mapResults[input.name].coverage = input.coverage
        state.allResults.mapResults[input.name].mapped_sequences = input.mapped_sequences
    },
};
