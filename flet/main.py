import flet as ft

class TodoApp(ft.Column):
    def __init__(self):
        super().__init__()
        self._tip = ft.Text("Today todos")
        self._text_field = ft.TextField(on_submit=self._on_add_todo, expand=True)
        self._add_button = ft.IconButton(icon=ft.icons.ADD, on_click=self._on_add_todo)
        self._todos_list = ft.Column()

        self.controls = [
            self._tip,
            ft.Row([
                self._text_field,
                self._add_button
            ]),
            self._todos_list
        ]

    def _on_add_todo(self, e):
        new_item = Todo(self._text_field.value, self._delete_todo)
        self._todos_list.controls.append(new_item)
        self._text_field.value = ""
        self.update()

    def _delete_todo(self, todo):
        self._todos_list.controls.remove(todo)
        self.update()

class Todo(ft.Column):
    def __init__(self, text, delete_todo):
        super().__init__()
        self._todo = ft.Checkbox(label=text, expand=True)
        self._delete_todo = delete_todo
        self._edit_text_field = ft.TextField(on_submit=self._on_save_todo, expand=True)
        self._display_view = ft.Row(
            alignment=ft.MainAxisAlignment.SPACE_BETWEEN,
            controls=[
                self._todo,
                ft.Row([
                    ft.IconButton(icon=ft.icons.CREATE_OUTLINED, on_click=self._on_edit_todo),
                    ft.IconButton(icon=ft.icons.DELETE_OUTLINE_OUTLINED, on_click=self._on_delete_todo)
                ])
            ]
        )
        self._edit_view = ft.Row(
            visible=False,
            alignment=ft.MainAxisAlignment.SPACE_BETWEEN,
            controls=[
                self._edit_text_field,
                ft.IconButton(icon=ft.icons.SAVE_OUTLINED, on_click=self._on_save_todo)
            ]
        )
        
        self.controls = [
            self._display_view,
            self._edit_view
        ]

    def _on_edit_todo(self, e):
        self._edit_text_field.value = self._todo.label
        self._display_view.visible = False
        self._edit_view.visible = True
        self.update()

    def _on_save_todo(self, e):
        self._todo.label = self._edit_text_field.value
        self._edit_view.visible = False
        self._display_view.visible = True
        self.update()

    def _on_delete_todo(self, e):
        self._delete_todo(self)

def main(page: ft.Page):
    todo_app = TodoApp()

    page.add(todo_app)

ft.app(main)
