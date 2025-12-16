import { writable } from "svelte/store";

export const user = writable(null);

export function login(username, password) {
    if (username === "admin" && password === "1234") {
        const userData = { username };
        localStorage.setItem("user", JSON.stringify(userData));
        user.set(userData);
        return true;
    }
    return false;
}

export function logout() {
    localStorage.removeItem("user");
    user.set(null);
}

export function loadUser() {
    const stored = localStorage.getItem("user");
    if (stored) {
        user.set(JSON.parse(stored));
    }
}
