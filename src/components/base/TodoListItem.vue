<script setup lang="ts">
import { Todo } from "@/bindings";

const props = defineProps<{
  todoListItem: Todo;
  checkable: boolean;
  checked: boolean;
}>();
const emit = defineEmits<{
  checkTodo: [];
  updateTitle: [title: string];
}>();
</script>

<template>
  <div class="flex cursor-pointer flex-row items-center gap-3">
    <div class="flex flex-grow flex-col">
      <div class="flex flex-row items-center gap-2">
        <input
          type="checkbox"
          class="checkbox-primary checkbox"
          :disabled="!props.checkable"
          :checked="props.checked"
          @change="
            (e) => {
              const target = e.target as HTMLInputElement;
              if (target && target.checked) {
                emit('checkTodo');
                target.checked = false;
              }
            }
          "
        />
        <input
          type="text"
          placeholder="Todo Title"
          class="input input-sm input-ghost w-2/4 flex-grow"
          :value="props.todoListItem.title"
          @input="
            (e) => {
              const target = e.target as HTMLInputElement;
              emit('updateTitle', target.value);
            }
          "
        />
        <div class="h-1/4 whitespace-nowrap text-base">
          {{
            (todoListItem.lapTime !== null &&
              new Date(todoListItem.lapTime).toISOString().substring(11, 22)) ||
            "--:--:--.--"
          }}
          / {{ todoListItem?.elapsedTime?.toString() || "--" }} 分
        </div>
      </div>
      <div class="flex justify-end">
        <input
          v-if="todoListItem?.branchName !== undefined"
          type="text"
          class="input input-xs input-ghost w-48 text-right text-xs text-slate-400"
          :value="todoListItem?.branchName"
        />
      </div>
    </div>
  </div>
</template>
