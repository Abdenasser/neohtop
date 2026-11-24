<script>
  import "../app.css";
  import { browser } from "$app/environment";
  import { uiLocale } from "$lib/i18n";

  $: if (browser) document.documentElement.lang = $uiLocale;
</script>

<slot />
