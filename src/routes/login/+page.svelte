<script>
  import {login, user} from "$lib/auth"
  import {goto} from "$app/navigation"
  let username = "";
  let password = "";
  let error = "";

  $: if ($user){
    goto("/dashboard");
  }

  function submitLogin() {
    const  ok = login(username, password)
    if (ok) goto("/dashboard");
    else error = "Username and password wrong !!"
    // console.log("userbane:", username);
    // console.log("Password:", password);
  }
</script>

<div class="min-h-screen flex items-center justify-center bg-gray-100 dark:bg-gray-900">
  <div class="w-full max-w-md bg-white dark:bg-gray-800 rounded-xl shadow-lg p-8">

    <h2 class="text-3xl font-bold text-center text-gray-800 dark:text-white mb-8">
      Login
    </h2>

    <form on:submit|preventDefault={submitLogin} class="space-y-6">

      <!-- Email -->
      <div>
        <label class="text-gray-700 dark:text-gray-300 text-sm font-medium">Email</label>
        <input
                type="text"
                bind:value={username}
                required
                class="mt-1 w-full px-4 py-2 border rounded-lg bg-gray-50 dark:bg-gray-700
						border-gray-300 dark:border-gray-600 text-gray-900 dark:text-white
						focus:ring-2 focus:ring-blue-500 focus:outline-none"
                placeholder="username"
        />
      </div>

      <!-- Password -->
      <div>
        <label class="text-gray-700 dark:text-gray-300 text-sm font-medium">Password</label>
        <input
                type="password"
                bind:value={password}
                required
                class="mt-1 w-full px-4 py-2 border rounded-lg bg-gray-50 dark:bg-gray-700
						border-gray-300 dark:border-gray-600 text-gray-900 dark:text-white
						focus:ring-2 focus:ring-blue-500 focus:outline-none"
                placeholder="••••••••"
        />
      </div>

      <button
              class="w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 rounded-lg
					transition-colors duration-200"
      >
        Sign In
      </button>

    </form>

    <p class="mt-6 text-center text-gray-600 dark:text-gray-400 text-sm">
      Don't have an account?
      <a href="/register" class="text-blue-600 hover:underline">Register</a>
    </p>
  </div>
</div>
