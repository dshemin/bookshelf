export interface PaginatedData<T> {
    data: T[],
    cursor?: string,
}
