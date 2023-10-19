import { combine, createEffect, createStore } from "effector";
import type { Storage } from "shared/api/storage";
import api from "shared/api";

export const getStorageListFx = createEffect((cursor?: string) => api.storage.list(cursor));
export const $getStorageListFxLoading = getStorageListFx.pending;

export const $storages = createStore<Storage[]>([])
    .on(getStorageListFx.doneData, (_, storage) => storage.data);

export const $storagesList = combine($storages, (storages) => Object.values(storages));
export const $storagesListEmpty = $storagesList.map((list) => list.length === 0);
