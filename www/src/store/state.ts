import {
    DatacinError,
    AllResults,
} from "@/types";

export interface RootState {
  errors: DatacinError[]
  allResults: AllResults
}
