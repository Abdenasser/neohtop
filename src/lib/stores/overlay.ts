import { writable } from "svelte/store";

type OverlayType =
  | "pagination"
  | "refresh"
  | "columns"
  | "theme"
  | "searchHelp"
  | "filters"
  | "status"
  | null;

function createOverlayStore() {
  const { subscribe, set } = writable<OverlayType>(null);

  return {
    subscribe,
    open: (overlayType: OverlayType) => set(overlayType),
    close: () => set(null),
    isOpen: (overlayType: OverlayType) => {
      let currentValue: OverlayType = null;
      subscribe((value) => (currentValue = value))();
      return currentValue === overlayType;
    },
  };
}

export const overlayStore = createOverlayStore();
export default overlayStore;
