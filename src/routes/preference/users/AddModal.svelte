<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";

  import { onMount } from "svelte";

  import { invoke } from "@tauri-apps/api/core";
  import { createEventDispatcher } from "svelte";
  import { Input, Label } from "flowbite-svelte";
  const dispatch = createEventDispatcher();
  function isCloseModal() {
    dispatch("toggle");
  }

  export let fetchUsers: () => void;

  let name = "";
  let email = "";
  let username = "";
  let password = "";

  let loading = false;

  function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    submitUser();
  }

  async function submitUser() {
    loading = true;
    try {
      await invoke("create_new_user", {
        newUser: {
          name,
          username,
          email,
          password,
        },
      });

      name = "";
      email = "";
      username = "";
      password = "";

      fetchUsers();
      isCloseModal();
    } catch (err) {
      console.error("Error adding user:", err);
      fetchUsers();
    } finally {
      loading = false;
      goto($page.url.pathname);
    }
  }
</script>

<div class="wrapper-modal">
  <div class="close">
    <button onclick={isCloseModal}>Close</button>
  </div>
  <div class="modal">
    <form onsubmit={handleSubmit}>
      <Label class="mb-2 font-bold text-lg">Add New User</Label>
      <Label class="text-lg">Name</Label>
      <Input bind:value={name} placeholder="Name" class="mb-4" />
      <Label class="text-lg">Email</Label>
      <Input bind:value={email} placeholder="Email" class="mb-4" />
      <Label class="bold text-lg">Username</Label>
      <Input bind:value={username} placeholder="Username" class="mb-4" />
      <Label class="text-lg">Password</Label>
      <Input
        bind:value={password}
        placeholder="Password"
        type="password"
        class="mb-4"
      />
      <button
        class="bg-blue-500 text-white px-4 py-2 rounded"
        disabled={loading}
      >
        {#if loading}
          <div role="status">
            <svg
              aria-hidden="true"
              class="w-8 h-8 text-neutral-tertiary animate-spin fill-brand"
              viewBox="0 0 100 101"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                fill="currentColor"
              />
              <path
                d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                fill="currentFill"
              />
            </svg>
            <span class="sr-only">Loading...</span>
          </div>
        {:else}
          Add User
        {/if}
      </button>
    </form>
  </div>
</div>

<style>
  .close {
    position: absolute;
    top: 10px;
    right: 10px;
    background-color: white;
    padding: 5px 10px;
    border-radius: 10px;
    border: none;
    font-size: 20px;
    cursor: pointer;
  }
  .close:hover {
    background-color: #f0f0f0;
  }

  .wrapper-modal {
    position: fixed; /* ✅ Ubah dari absolute ke fixed untuk full screen */
    transition: all 0.3s ease-in-out;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: rgba(
      0,
      0,
      0,
      0.8
    ); /* ✅ Background solid tanpa opacity pada wrapper */
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background-color: white;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    z-index: 1001;
    max-width: 500px;
    width: 90%;
    max-height: 80vh;
    overflow-y: auto;
    /* ✅ Hapus opacity dari modal */
  }
</style>
