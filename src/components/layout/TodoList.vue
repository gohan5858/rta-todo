<script setup lang="ts">
import { TodoList } from "@/bindings";
import TodoListItem from "@base/TodoListItem.vue";
import { PhDotsSixVertical } from "@phosphor-icons/vue";
import { VueDraggable } from "vue-draggable-plus";

const emit = defineEmits<{
  checkTodo: [parentId: string | null];
  updateTodoTitle: [todoList: TodoList];
}>();

const props = defineProps<{
  todoList: TodoList;
}>();

</script>

<template>
  <div class="flex flex-col gap-1">
    <TodoListItem
      v-for="checkedTodo in props.todoList.checked_todos"
      :todo-list-item="checkedTodo"
      :checkable="false"
      :checked="true"
    />
    <VueDraggable
      tag="ul"
      v-model="props.todoList.unchecked_todos"
      :animation="150"
      handle="#handle"
    >
      <li
        v-for="(uncheckedTodo, index) in props.todoList.unchecked_todos"
        class="flex flex-row gap-3"
      >
        <TodoListItem
          class="flex-grow"
          :todo-list-item="uncheckedTodo"
          :checkable="index === 0"
          :checked="false"
          @check-todo="() => emit('checkTodo', null)"
          @update-title="
            (title) => {
              props.todoList.unchecked_todos[index].title = title;
              emit('updateTodoTitle', props.todoList);
            }
          "
        />
        <PhDotsSixVertical id="handle" class="cursor-move" :size="32" />
      </li>
    </VueDraggable>
  </div>
</template>
