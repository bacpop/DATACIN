import {
    DatacinError,
    AllResults,
    State,
} from "@/types";

export interface RootState {
  errors: DatacinError[]
  allResults: AllResults
  workerState: State
}
