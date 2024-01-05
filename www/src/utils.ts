import { RootState } from "@/store/state";

export const emptyState = (): RootState => ({
    refSet: null,
    allResults: {
        mapResults: {},
    },
    workerState: {
        worker: null,
        workerResult: null,
    },
});
