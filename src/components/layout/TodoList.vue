<script setup lang="ts">
import { Todo } from "@/bindings";
import TodoListItem from "@base/TodoListItem.vue";
import { SortableEvent, VueDraggable } from "vue-draggable-plus";

const emit = defineEmits<{
  checkedTodo: [index: number, checked: boolean];
}>();

const checkedTodoList = defineModel<Todo[]>("checkedTodoList", {
  required: true,
});
const uncheckedTodoList = defineModel<Todo[]>("uncheckedTodoList", {
  required: true,
});

const onSortedTodoList = (e: SortableEvent) => {
  if (
    e.newIndex === undefined ||
    e.oldIndex === undefined ||
    uncheckedTodoList.value.length <= 1
  ) {
    return;
  }

  const dragItem = uncheckedTodoList.value.at(e.oldIndex);
  if (!dragItem) {
    return;
  }

  // チェック可能なtodoがドラッグされた場合
  if (dragItem.checkable) {
    dragItem.checkable = false;
    const nextTop = uncheckedTodoList.value.at(1);
    nextTop && (nextTop.checkable = true);
  }
  // チェック不可能なtodoが一番上にドラッグされた場合
  else if (e.newIndex === 0) {
    uncheckedTodoList.value.forEach((item) => (item.checkable = false));
    const nextTop = uncheckedTodoList.value.at(e.oldIndex);
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
