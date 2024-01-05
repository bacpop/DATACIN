import { ActionContext } from "vuex";
import { RootState } from "@/store/state";

export default {
    sendMessageToWorker(context: ActionContext<RootState, RootState>, message: string) {
        const { commit, state } = context;
        if (state.workerState.worker) {
            state.workerState.worker.postMessage(message);
            state.workerState.worker.onmessage = event => {
                console.log("Worker returned" + event.data);
                commit('set_worker_result', event.data);
            };
            state.workerState.worker.onerror = event => {
                console.error('Error in worker:', event.message);
            };
        }
    },
    async processRef(context: ActionContext<RootState, RootState>, acceptFiles: Array<File>) {
        const { commit, state } = context;
        console.log("Ref file uploaded")
        acceptFiles.forEach((file: File) => {
            if (state.workerState.worker) {
                state.workerState.worker.postMessage({ref: true, filename: file.name});
                state.workerState.worker.onmessage = () => {
                    console.log(file.name + " has been indexed");
                    commit("addRef", file.name);
                };
            }
        });
    },
    async processQuery(context: ActionContext<RootState, RootState>, acceptFiles: Array<File>) {
        const { commit, state } = context;
        console.log("Query files uploaded")
        acceptFiles.forEach((file: File) => {
            commit("addQueryFile", file.name);
            if (state.workerState.worker) {
                state.workerState.worker.postMessage({map: true, filename: file.name});
                state.workerState.worker.onmessage = () => {
                    console.log(file.name + " has been mapped");
                    commit("setMapped", file.name);
                };
            }
        });
    },
};
