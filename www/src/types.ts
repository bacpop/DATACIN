export type Dict<T> = Record<string, T>

export interface State {
    worker: Worker | null;
    workerResult: string | null;
}

export interface Isolate {
    filename: string
    mapping?: MapResults
}

export interface Coor {
    position: number
    base: string
}

// For testing
export interface MapResults {
    variants: Array<number>
}

/*
export interface MapResults {
    variants: Array<Coor>
}
*/

export interface AllResults {
    mapResults: Dict<Isolate>
}


export interface DatacinError {
    error: string,
    detail?: string
}


