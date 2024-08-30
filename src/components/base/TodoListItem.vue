<script setup lang="ts">
import { TodoItem } from "@/todoItem";

const todoListItem = defineModel<TodoItem>("todoListItem", {
  required: true,
});
const emit = defineEmits<{
  checkedTodo: [checked: boolean];
}>();
</script>

<template>
  <div class="flex cursor-pointer flex-row items-center gap-3">
    <div class="flex flex-grow flex-col">
      <div class="flex flex-row items-center gap-2">
        <input
          type="checkbox"
          :disabled="!todoListItem?.checkable"
          class="checkbox-primary checkbox"
          @change="emit('checkedTodo', todoListItem.checked)"
          v-model="todoListItem.checked"
        />
        <input
          type="text"
          placeholder="Todo Title"
          class="input input-sm input-ghost w-2/4 flex-grow"
          v-model="todoListItem.title"
        />
        <div class="h-1/4 whitespace-nowrap text-base">
          {{
            todoListItem?.lapTime?.toISOString().substring(11, 22) ||
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
