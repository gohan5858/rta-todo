<script setup lang="ts">
import { TodoItem } from "@/todoItem";
import TodoListItem from "@base/TodoListItem.vue";
import { VueDraggable } from "vue-draggable-plus";

const emit = defineEmits<{
  checkedTodo: [index: number, checked: boolean];
}>();

const checkedTodoList = defineModel<TodoItem[]>("checkedTodoList", {
  required: true,
});
const uncheckedTodoList = defineModel<TodoItem[]>("uncheckedTodoList", {
  required: true,
});
</script>

<template>
  <div class="flex flex-col gap-1">
    <TodoListItem
      v-for="(_, index) in checkedTodoList"
      v-model:todo-list-item="checkedTodoList[index]"
    />
    <VueDraggable ref="el" v-model="uncheckedTodoList" :animation="150">
      <TodoListItem
        v-for="(_, index) in uncheckedTodoList"
        @checked-todo="(checked) => emit('checkedTodo', index, checked)"
        v-model:todo-list-item="uncheckedTodoList[index]"
      />
    </VueDraggable>
  </div>
</template>
