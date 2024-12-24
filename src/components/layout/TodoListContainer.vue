<script setup lang="ts">
import { events, Todo, TodoList } from "@/bindings";
import TodoListItem from "@base/TodoListItem.vue";
import { PhDotsSixVertical } from "@phosphor-icons/vue";
import { onMounted, ref } from "vue";
import { VueDraggable } from "vue-draggable-plus";

const emit = defineEmits<{
  checkTodo: [parentId: string | null];
  updateTodoList: [todoList: TodoList];
}>();

const props = defineProps<{
  todoList: TodoList;
  maxNestLevel: number;
}>();

const isDragging = defineModel<boolean>();

const isPaused = ref(true);
onMounted(() => {
  events.updaterIsPaused.listen((events) => {
    isPaused.value = events.payload;
  });
});
</script>

<template>
  <div class="flex flex-col gap-1">
    <div
      class="flex flex-col gap-1"
      v-for="checkedTodo in props.todoList.checked_todos"
    >
      <TodoListItem
        class="pr-7"
        :todo-list-item="checkedTodo"
        :checkable="false"
        :checked="true"
      />
      <div class="p-2">
        <div v-for="subTodo in checkedTodo.subTodoList.checked_todos">
          <TodoListItem
            class="pl-4 pr-7"
            :todo-list-item="subTodo"
            :checkable="false"
            :checked="true"
          />
        </div>
        <div v-for="subTodo in checkedTodo.subTodoList.unchecked_todos">
          <TodoListItem
            class="pl-4 pr-7"
            :todo-list-item="subTodo"
            :checkable="false"
            :checked="false"
          />
        </div>
      </div>
    </div>
    <VueDraggable
      class="flex flex-col gap-1"
      :class="isDragging ? 'min-h-7 outline outline-1' : ''"
      :model-value="props.todoList.unchecked_todos"
      @start="() => (isDragging = true)"
      @end="() => (isDragging = false)"
      @update:model-value="
        (unchecked_todos: Todo[]) => {
          props.todoList.unchecked_todos = unchecked_todos;
          emit('updateTodoList', props.todoList);
        }
      "
      group="todoList"
      :animation="150"
      handle="#handle"
    >
      <div
        v-for="(uncheckedTodo, index) in props.todoList.unchecked_todos"
        :key="uncheckedTodo.id"
        class="flex flex-col"
      >
        <div class="flex flex-row gap-1">
          <TodoListItem
            class="flex-grow"
            :todo-list-item="uncheckedTodo"
            :checkable="!isPaused && index === 0"
            :checked="false"
            @check-todo="
              () =>
                emit(
                  'checkTodo',
                  props.maxNestLevel == 1 ? null : uncheckedTodo.id,
                )
            "
            @update-title="
              (title) => {
                props.todoList.unchecked_todos[index].title = title;
                emit('updateTodoList', props.todoList);
              }
            "
          />
          <div class="flex items-center">
            <PhDotsSixVertical
              id="handle"
              class="cursor-ns-resize"
              :size="24"
            />
          </div>
        </div>
        <TodoListContainer
          class="p-2 pl-4"
          v-if="props.maxNestLevel > 0"
          v-model="isDragging"
          :todo-list="uncheckedTodo.subTodoList"
          :max-nest-level="props.maxNestLevel - 1"
          @check-todo="async (parentId) => emit('checkTodo', parentId)"
          @update-todo-list="
            (todoList: TodoList) => {
              props.todoList.unchecked_todos[index].subTodoList = todoList;
              emit('updateTodoList', props.todoList);
            }
          "
        />
      </div>
    </VueDraggable>
  </div>
</template>
