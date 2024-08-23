<script setup lang="ts">
import TodoListItem from "@base/TodoListItem.vue";

const props = defineProps<{
  todoList: {
    title: string;
    lapTime: string;
    checkable: boolean;
    branchName?: string;
  }[];
}>();
const emit = defineEmits<{
  checkedTodo: [index: number];
}>();
</script>

<template>
  <div class="flex flex-col gap-1">
    <TodoListItem
      @checked-todo="emit('checkedTodo', index)"
      v-for="(
        { title, lapTime, checkable, branchName }, index
      ) in props.todoList"
      :title="title"
      :lap-time="lapTime"
      :branch-name="branchName"
      :checkable="checkable"
    />
  </div>
</template>
