/* eslint-disable */
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
    interface Window {
        __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
    }
}

// Function avoids 'window not defined' in SSR
const invoke = () => window.__TAURI_INVOKE__;

export function startTimer() {
    return invoke()<null>("start_timer")
}

export function pauseTimer() {
    return invoke()<null>("pause_timer")
}

export function resumeTimer() {
    return invoke()<null>("resume_timer")
}

export function resetTimer() {
    return invoke()<null>("reset_timer")
}

export function getCurrentTime() {
    return invoke()<number>("get_current_time")
}

export function loadData() {
    return invoke()<SaveData>("load_data")
}

export type SaveData = { todoLists: TodoList[] }
export type Todo = { id: number; lap_time: number | null; elapsed_time: number | null; checked: boolean; checkable: boolean; branch_name: string | null }
export type TodoList = { id: number; title: string; completed: boolean; todos: Todo[] }
