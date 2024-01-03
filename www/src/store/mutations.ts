import { RootState } from "@/store/state";
import {
    DatacinError,
    Isolate,
} from "@/types";

export default {
    addError(state: RootState, payload: DatacinError) {
        state.errors.push(payload);
    },
    addFile(state: RootState, name: string) {
        if (!state.allResults.mapResults[name]) {
            state.allResults.mapResults[name] = {
                filename: name
            };
        }
    },
    setIsolateResults(state: RootState, input: Isolate) {
        state.allResults.mapResults[input.filename] = input
    },
};
