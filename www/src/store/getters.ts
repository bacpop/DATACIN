import { RootState } from "@/store/state";
import { GetterTree } from "vuex";

export const getters: GetterTree<RootState, RootState> = {
    refProcessed(state: RootState) {
        console.log("Ref processed (getters)")
        return state.refSet == null ? false : true;
    },
    refName(state: RootState) {
        return state.refSet == null ? "" : state.refSet;
    },
    queryProcessed(state: RootState) {
        const first_input: string = Object.keys(state.allResults.mapResults)[0]
        return state.allResults.mapResults[first_input] ? state.allResults.mapResults[first_input].mapped : false;
    }
}