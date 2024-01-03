import { ActionContext } from "vuex";
import { RootState } from "@/store/state";

export default {
    async processFiles(context: ActionContext<RootState, RootState>, acceptFiles: Array<File>) {
        const { commit } = context;
        acceptFiles.forEach((file: File) => {
            commit("addFile", { name: file.name });
            const worker = new Worker("./worker.js");
            worker.onmessage = (event) => {
                commit("setIsolateResults", event.data);
            };
            worker.postMessage({ filename: file.name, fileObject: file });
        });
    },
};
