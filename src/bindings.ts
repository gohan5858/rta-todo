// @ts-nocheck

// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

/** user-defined commands **/


export const commands = {
async initiateTimer(projectId: string) : Promise<number> {
    return await TAURI_INVOKE("initiate_timer", { projectId });
},
async pauseTimer() : Promise<null> {
    return await TAURI_INVOKE("pause_timer");
},
async resumeTimer() : Promise<null> {
    return await TAURI_INVOKE("resume_timer");
},
async getCurrentTime() : Promise<number> {
    return await TAURI_INVOKE("get_current_time");
},
async loadData() : Promise<SaveData> {
    return await TAURI_INVOKE("load_data");
},
async setTitle(projectId: string, title: string) : Promise<null> {
    return await TAURI_INVOKE("set_title", { projectId, title });
},
async setTheme(theme: string) : Promise<null> {
    return await TAURI_INVOKE("set_theme", { theme });
},
async setIsAutoStart(isAutoStart: boolean) : Promise<null> {
    return await TAURI_INVOKE("set_is_auto_start", { isAutoStart });
},
async setIsNotificationOfDeadline(isNotificationOfDeadline: boolean) : Promise<null> {
    return await TAURI_INVOKE("set_is_notification_of_deadline", { isNotificationOfDeadline });
},
async setIsNotificationExceededGoalLapTime(isNotificationExceededGoalLapTime: boolean) : Promise<null> {
    return await TAURI_INVOKE("set_is_notification_exceeded_goal_lap_time", { isNotificationExceededGoalLapTime });
},
async setIsCompleteProject(projectId: string, isComplete: boolean) : Promise<null> {
    return await TAURI_INVOKE("set_is_complete_project", { projectId, isComplete });
},
async addProject(title: string, deadline: string | null) : Promise<null> {
    return await TAURI_INVOKE("add_project", { title, deadline });
},
async fetchProject(projectId: string) : Promise<Project> {
    return await TAURI_INVOKE("fetch_project", { projectId });
},
async removeProject(projectId: string) : Promise<null> {
    return await TAURI_INVOKE("remove_project", { projectId });
},
async addTodo(projectId: string) : Promise<TodoList> {
    return await TAURI_INVOKE("add_todo", { projectId });
},
async removeTodo(projectId: string) : Promise<TodoList> {
    return await TAURI_INVOKE("remove_todo", { projectId });
},
async updateTodoList(projectId: string, todoList: TodoList) : Promise<null> {
    return await TAURI_INVOKE("update_todo_list", { projectId, todoList });
},
async goToNextTodo(projectId: string, parentId: string | null) : Promise<TodoList> {
    return await TAURI_INVOKE("go_to_next_todo", { projectId, parentId });
},
async updateCurrentElapsedTime(projectId: string, currentElapsedTime: number) : Promise<null> {
    return await TAURI_INVOKE("update_current_elapsed_time", { projectId, currentElapsedTime });
},
async getCurrentElapsedTime(projectId: string) : Promise<number> {
    return await TAURI_INVOKE("get_current_elapsed_time", { projectId });
},
async resetCurrentElapsedTime() : Promise<null> {
    return await TAURI_INVOKE("reset_current_elapsed_time");
}
}

/** user-defined events **/


export const events = __makeEvents__<{
updaterIsPaused: UpdaterIsPaused
}>({
updaterIsPaused: "updater-is-paused"
})

/** user-defined constants **/



/** user-defined types **/

export type Project = { id: string; title: string; deadline: string | null; currentElapsedTime: number; completed: boolean; todoList: TodoList }
export type SaveData = { theme: string; isAutoStart: boolean; isNotificationOfDeadline: boolean; isNotificationExceededGoalLapTime: boolean; projects: Project[] }
export type Todo = { id: string; title: string; lapTime: number | null; elapsedTime: number | null; branchName: string | null; subTodoList: TodoList }
export type TodoList = { checked_todos: Todo[]; unchecked_todos: Todo[] }
export type UpdaterIsPaused = boolean

/** tauri-specta globals **/

import {
	invoke as TAURI_INVOKE,
	Channel as TAURI_CHANNEL,
} from "@tauri-apps/api/core";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindow as __WebviewWindow__ } from "@tauri-apps/api/webviewWindow";

type __EventObj__<T> = {
	listen: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
	once: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
	emit: null extends T
		? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
		: (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type Result<T, E> =
	| { status: "ok"; data: T }
	| { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
	mappings: Record<keyof T, string>,
) {
	return new Proxy(
		{} as unknown as {
			[K in keyof T]: __EventObj__<T[K]> & {
				(handle: __WebviewWindow__): __EventObj__<T[K]>;
			};
		},
		{
			get: (_, event) => {
				const name = mappings[event as keyof T];

				return new Proxy((() => {}) as any, {
					apply: (_, __, [window]: [__WebviewWindow__]) => ({
						listen: (arg: any) => window.listen(name, arg),
						once: (arg: any) => window.once(name, arg),
						emit: (arg: any) => window.emit(name, arg),
					}),
					get: (_, command: keyof __EventObj__<any>) => {
						switch (command) {
							case "listen":
								return (arg: any) => TAURI_API_EVENT.listen(name, arg);
							case "once":
								return (arg: any) => TAURI_API_EVENT.once(name, arg);
							case "emit":
								return (arg: any) => TAURI_API_EVENT.emit(name, arg);
						}
					},
				});
			},
		},
	);
}
