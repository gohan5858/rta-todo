<script setup lang="ts">
import { TodoItem } from "@/todoItem";
import TodoListItem from "@base/TodoListItem.vue";
import { SortableEvent, VueDraggable } from "vue-draggable-plus";

const emit = defineEmits<{
  checkedTodo: [index: number, checked: boolean];
}>();

const checkedTodoList = defineModel<TodoItem[]>("checkedTodoList", {
  required: true,
});
const uncheckedTodoList = defineModel<TodoItem[]>("uncheckedTodoList", {
  required: true,
});

const onSortedTodoList = (e: SortableEvent) => {
  if (e.oldIndex === undefined) {
    return;
  }

  uncheckedTodoList.value.forEach((item) => (item.checkable = false));
  if (e.newIndex === 0) {
    const nextTop = uncheckedTodoList.value.at(e.oldIndex);
    nextTop && (nextTop.checkable = true);
  } else {
    const nextTop = uncheckedTodoList.value.at(1);
    nextTop && (nextTop.checkable = true);
  }
};
</script>

<template>
  <div class="flex flex-col gap-1">
    <TodoListItem
      v-for="(_, index) in checkedTodoList"
      v-model:todo-list-item="checkedTodoList[index]"
    />
    <VueDraggable
      ref="el"
      v-model="uncheckedTodoList"
      :animation="150"
      :onEnd="(e) => onSortedTodoList(e)"
    >
      <TodoListItem
        v-for="(_, index) in uncheckedTodoList"
        @checked-todo="(checked) => emit('checkedTodo', index, checked)"
        v-model:todo-list-item="uncheckedTodoList[index]"
      />
    </VueDraggable>
  </div>
</template>
