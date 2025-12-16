import { redirect } from '@sveltejs/kit';

// console.log("adaslkjdadjlsjskl")
//
// export const handle = async ({ event, resolve }) => {
//     const response = await resolve(event);
//
//     // Jika route tidak ditemukan
//     if (response.status === 404) {
//         console.warn("404 DETECTED:", event.url.pathname);
//
//         // Jika berjalan di Tauri, kirim log ke Rust
//         if (typeof window !== 'undefined' && window.__TAURI__) {
//             window.__TAURI__.invoke("log_404", {
//                 path: event.url.pathname
//             });
//         }
//     }
//
//     return response;
// };

