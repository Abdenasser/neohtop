import { render } from "svelte/server";
import Site from "./Site.svelte";
import "./styles.css";
export function renderSite(page) {
  return render(Site, { props: { page } });
}
