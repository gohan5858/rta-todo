export interface TodoListItem {
  title: string;
  lapTime: Date | undefined;
  elapsedTime: number | undefined;
  checked: boolean;
  checkable: boolean;
  branchName?: string;
}
