// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
    namespace App {
        // interface Error {}
        // interface Locals {}
        // interface PageData {}
        // interface PageState {}
        // interface Platform {}
    }

    interface Window {
        user: User | null;
        userPromise: Promise<User | null> | null;
        refreshUser: () => Promise<User | null>;
        getUser: () => Promise<User | null>;
    }
}

export class User {
    id: string;
    name: string;
}
