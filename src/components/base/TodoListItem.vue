<script setup lang="ts">
import { ref } from "vue";

const checked = ref(false);

const props = defineProps<{
  title: string;
  lapTime: string;
  checkable: boolean;
  branchName?: string;
}>();
const emit = defineEmits<{
  checkedTodo: [];
}>();
</script>

<template>
  <div class="flex cursor-pointer flex-row items-center gap-3">
    <div class="flex flex-grow flex-col">
      <div class="flex flex-row items-center gap-2">
        <input
          type="checkbox"
          :disabled="!props.checkable"
          class="checkbox-primary checkbox"
          @change="emit('checkedTodo')"
          v-model="checked"
        />
        <input
          type="text"
          placeholder="Todo Title"
          class="input input-sm input-ghost w-2/4 flex-grow"
          :value="props.title"
        />
        <div class="h-1/4 text-base">{{ props.lapTime }}</div>
      </div>
      <div class="flex justify-end">
        <input
          v-if="props.branchName !== undefined"
          type="text"
          class="input input-xs input-ghost w-48 text-right text-xs text-slate-400"
          :value="props.branchName"
        />
      </div>
    </div>
  </div>
</template>
