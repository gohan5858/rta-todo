<script setup lang="ts">
import { TodoItem } from "@/todoItem";
import { onMounted, ref } from "vue";

const currentChecked = ref(false);
const props = defineProps<{
  todoList: TodoItem;
  checked: boolean;
}>();
const emit = defineEmits<{
  checkedTodo: [checked: boolean];
}>();

onMounted(() => {
  currentChecked.value = props.checked;
});
</script>

<template>
  <div class="flex cursor-pointer flex-row items-center gap-3">
    <div class="flex flex-grow flex-col">
      <div class="flex flex-row items-center gap-2">
        <input
          type="checkbox"
          :disabled="!props.todoList.checkable"
          class="checkbox-primary checkbox"
          @change="emit('checkedTodo', currentChecked)"
          v-model="currentChecked"
        />
        <input
          type="text"
          placeholder="Todo Title"
          class="input input-sm input-ghost w-2/4 flex-grow"
          :value="props.todoList.title"
        />
        <div class="h-1/4 whitespace-nowrap text-base">
          {{
            props.todoList.lapTime?.toISOString().substring(11, 22) ||
            "--:--:--.--"
          }}
          / {{ props.todoList.elapsedTime?.toString() || "--" }} 分
        </div>
      </div>
      <div class="flex justify-end">
        <input
          v-if="props.todoList.branchName !== undefined"
          type="text"
          class="input input-xs input-ghost w-48 text-right text-xs text-slate-400"
          :value="props.todoList.branchName"
        />
      </div>
    </div>
  </div>
</template>
