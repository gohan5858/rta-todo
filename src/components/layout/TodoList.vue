<script setup lang="ts">
import { TodoItem } from "@/todoItem";
import TodoListItem from "@base/TodoListItem.vue";
import { Ref, ref, watch } from "vue";
import draggable from "vuedraggable";

const props = defineProps<{
  todoList: TodoItem[];
}>();
const emit = defineEmits<{
  checkedTodo: [index: number, checked: boolean];
}>();

const checkedTodoItems: Ref<TodoItem[]> = ref([]);
const uncheckedTodoItems: Ref<TodoItem[]> = ref([]);

watch(
  () => props.todoList,
  (newTodoList) => {
    checkedTodoItems.value.splice(0, checkedTodoItems.value.length);
    uncheckedTodoItems.value.splice(0, uncheckedTodoItems.value.length);

    checkedTodoItems.value.push(
      ...newTodoList
        .filter((todo) => todo.checked)
        .map((todo, index) => ({
          ...todo,
          index,
        })),
    );
    uncheckedTodoItems.value.push(
      ...newTodoList
        .filter((todo) => !todo.checked)
        .map((todo, index) => ({
          ...todo,
          index: index + checkedTodoItems.value.length,
        })),
    );

    console.table(
      newTodoList
        .filter((todo) => todo.checked)
        .map((todo, index) => ({
          ...todo,
          index,
        })),
    );
    console.table(
      newTodoList
        .filter((todo) => !todo.checked)
        .map((todo, index) => ({
          ...todo,
          index: index + checkedTodoItems.value.length,
        })),
    );
  },
  { deep: true, immediate: true },
);
</script>

<template>
  <div class="flex flex-col gap-1">
    <TodoListItem
      v-for="checkedTodo in checkedTodoItems"
      :todo-list="{
        title: checkedTodo.title,
        lapTime: checkedTodo.lapTime,
        elapsedTime: checkedTodo.elapsedTime,
        branchName: checkedTodo.branchName,
        checked: checkedTodo.checked,
        checkable: checkedTodo.checkable,
      }"
      :checked="checkedTodo.checked"
    />
    <draggable v-model="uncheckedTodoItems" item-key="index" tag="ul">
      <template #item="{ element: uncheckedTodo }">
        <TodoListItem
          @checked-todo="
            (checked) => emit('checkedTodo', uncheckedTodo.index, checked)
          "
          :todo-list="{
            title: uncheckedTodo.title,
            lapTime: uncheckedTodo.lapTime,
            elapsedTime: uncheckedTodo.elapsedTime,
            branchName: uncheckedTodo.branchName,
            checked: uncheckedTodo.checked,
            checkable: uncheckedTodo.checkable,
          }"
          :checked="uncheckedTodo.checked"
        />
      </template>
    </draggable>
  </div>
</template>
