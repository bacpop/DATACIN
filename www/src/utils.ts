import { RootState } from "@/store/state";

export const emptyState = (): RootState => ({
    errors: [],
    allResults: {
        mapResults: {},
    }
});
