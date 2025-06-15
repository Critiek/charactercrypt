// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
    declare namespace svelteHTML {
        interface HTMLAttributes<T> {
            onoutclick?: (event: CustomEvent) => void;
        }
    }
    namespace App {
        // interface Error {}
        // interface Locals {}
        // interface PageData {}
        // interface PageState {}
        // interface Platform {}
    }
}

export {};
