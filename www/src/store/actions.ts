import { ActionContext } from "vuex";
import { RootState } from "@/store/state";

export default {
    sendMessageToWorker(context: ActionContext<RootState, RootState>, message: string) {
        const { commit, state } = context;
        if (state.workerState.worker) {
            state.workerState.worker.postMessage(message);
            state.workerState.worker.onmessage = event => {
                commit('SET_WORKER_RESULT', event.data);
            };
            state.workerState.worker.onerror = event => {
                console.error('Error in worker:', event.message);
            };
        }
    },
    terminateWorker(context: ActionContext<RootState, RootState>) {
        const { commit, state } = context;
        if (state.workerState.worker) {
            state.workerState.worker.terminate();
            commit('SET_WORKER', null);
        }
    },
    async processFiles(context: ActionContext<RootState, RootState>, acceptFiles: Array<File>) {
        console.log("Files would be accepted here")
        /*
        const { commit, state } = context;
        acceptFiles.forEach((file: File) => {
            commit("addFile", { name: file.name });
            const worker = new Worker("./worker.js");
            worker.onmessage = (event) => {
                commit("setIsolateResults", event.data);
            };
            worker.postMessage({ filename: file.name, fileObject: file });
        });
        */
    },
};
