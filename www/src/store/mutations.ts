import { RootState } from "@/store/state";
import {
    DatacinError,
    Isolate,
} from "@/types";

export default {
    SET_WORKER(state: RootState, worker: Worker | null) {
        state.workerState.worker = worker;
    },
    SET_WORKER_RESULT(state: RootState, result: string) {
        state.workerState.workerResult = result;
    },
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
