<script setup lang="ts">
import { TodoItem } from "@/todoItem";
import draggableComponent from "vuedraggable";

const props = defineProps<{
  todoList: TodoItem[];
}>();
const emit = defineEmits<{
  checkedTodo: [index: number];
}>();

const todoItems = ref(
  props.todoList.map((todo, index) => ({ ...todo, index })),
);

watch(
  () => props.todoList,
  (newTodoList) => {
    todoItems.value = newTodoList.map((todo, index) => ({ ...todo, index }));
  },
  { deep: true },
);
</script>

<template>
  <div class="flex flex-col gap-1">
    <draggableComponent v-model="todoItems" item-key="index" tag="ul">
      <template #item="{ element }">
        <TodoListItem
          @checked-todo="emit('checkedTodo', element.index)"
          :title="element.title"
          :lap-time="
            element.lapTime?.toISOString().substring(11, 22) || '--:--:--.--'
          "
          :elapsed-time="element.elapsedTime?.toString() || '--'"
          :branch-name="element.branchName"
          :checkable="element.checkable"
        />
      </template>
    </draggableComponent>
  </div>
</template>
