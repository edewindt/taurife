<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  let path_name = "C:\\";
  let cdrive = [];
  let paths = [];
  onMount(async () => {
    cdrive = await invoke("return_cdrive", { thePath: "C:/" });
  });
  async function getnextdir(path) {
    paths = [...paths, path_name];
    if (paths.length > 1) {
      path_name += "\\" + path;
    } else {
      path_name += path;
    }

    cdrive = await invoke("return_cdrive", { thePath: path_name });
  }
  async function goback() {
    if (paths.length == 0) return;
    path_name = paths.pop();
    paths = paths;
    cdrive = await invoke("return_cdrive", {
      thePath: path_name,
    });
  }
  let crumb = "";
  function goto_breadcrumb(bread) {
    while (bread != crumb) {
      goback();
      crumb = path_name;
    }
    crumb = "";
  }
</script>

<div>
  <button on:click={goback}>Back</button>
  <h1>{path_name}</h1>
  <h2>Breadcrumbs:</h2>
  <div class="paths">
    {#each paths as p}
      <p
        class="path"
        on:click={() => {
          goto_breadcrumb(p);
        }}
      >
        {p}
      </p>
    {/each}
  </div>

  {#each cdrive as c}
    <button
      on:click={() => {
        getnextdir(c.name);
      }}
      >{#if c.type == "folder"}
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="18"
          height="18"
          viewBox="0 0 24 24"
          ><path
            d="M1 9v-7h6c1.695 1.942 2.371 3 4 3h12v4h-22zm-1 2l2 11h20l2-11h-24z"
          /></svg
        >
      {:else}
        <svg
          class="file"
          width="24"
          height="24"
          xmlns="http://www.w3.org/2000/svg"
          fill-rule="evenodd"
          clip-rule="evenodd"
          ><path
            d="M22 24h-20v-24h14l6 6v18zm-7-23h-12v22h18v-16h-6v-6zm1 5h4.586l-4.586-4.586v4.586z"
          /></svg
        >
      {/if}<span>{path_name}</span>{#if paths.length > 0}\{/if}{c.name}
    </button>
  {/each}
</div>

<style>
  h1 {
    padding-left: 1rem;
    font-family: Arial, Helvetica, sans-serif;
    font-weight: 500;
    margin: 1rem 0;
  }
  h2 {
    padding-left: 1rem;
  }

  button {
    padding: 0.85rem;
    padding-left: 1rem;
    font-size: 1rem;
    display: block;
    width: 100%;
    text-align: left;
    color: rgba(0, 0, 0, 0.749);
    background-color: rgba(245, 245, 245, 0.231);
    border: none;
    transition: 0.4s;
    cursor: pointer;
  }
  button:first-child {
    color: black;
    box-shadow: 0 0 3rem rgba(80, 34, 146, 0.258);
    background-color: white;
    position: sticky;
    top: 0;
    z-index: 2;
  }
  button:last-child {
    border-bottom: none;
  }
  button:hover {
    opacity: 1;
    color: black;
    box-shadow: 0 0 3rem rgba(80, 34, 146, 0.258);
  }
  button:first-child:hover {
    color: white;
    background-color: black;
  }
  span {
    background-color: rgba(255, 214, 127, 0.267);
    margin-left: 1rem;
  }
  .paths {
    padding: 1rem;
  }
  .file {
    scale: 0.85;
  }
  .path:hover {
    text-decoration: underline;
    cursor: pointer;
  }
</style>
