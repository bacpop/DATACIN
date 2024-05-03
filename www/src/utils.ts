import { RootState } from "@/store/state";

export const emptyState = (): RootState => ({
    refSet: null,
    allResults: {
        mapResults: {},
        ref: [],
    },
    workerState: {
        worker: null,
    },
});
