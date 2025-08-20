use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

impl Todo {
    fn new(id: usize, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}

#[derive(Default)]
struct TodoApp {
    todos: Vec<Todo>,
    new_todo_text: String,
    next_id: usize,
    save_file: PathBuf,
}

impl TodoApp {
    fn new() -> Self {
        let mut app = Self {
            todos: Vec::new(),
            new_todo_text: String::new(),
            next_id: 0,
            save_file: PathBuf::from("todos.json"),
        };
        app.load_todos();
        app
    }

    fn add_todo(&mut self) {
        if !self.new_todo_text.trim().is_empty() {
            let todo = Todo::new(self.next_id, self.new_todo_text.clone());
            self.todos.push(todo);
            self.next_id += 1;
            self.new_todo_text.clear();
            self.save_todos();
        }
    }

    fn toggle_todo(&mut self, id: usize) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
            self.save_todos();
        }
    }

    fn delete_todo(&mut self, id: usize) {
        self.todos.retain(|todo| todo.id != id);
        self.save_todos();
    }

    fn save_todos(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.todos) {
            let _ = fs::write(&self.save_file, json);
        }
    }

    fn load_todos(&mut self) {
        if let Ok(contents) = fs::read_to_string(&self.save_file) {
            if let Ok(todos) = serde_json::from_str::<Vec<Todo>>(&contents) {
                self.todos = todos;
                // Set next_id to be higher than any existing id
                self.next_id = self.todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            }
        }
    }

    fn clear_completed(&mut self) {
        self.todos.retain(|todo| !todo.completed);
        self.save_todos();
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📝 Todo List");
            ui.separator();

            // Add new todo section
            ui.horizontal(|ui| {
                ui.label("New todo:");
                let response = ui.text_edit_singleline(&mut self.new_todo_text);

                if (ui.button("Add").clicked()
                    || response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    && !self.new_todo_text.trim().is_empty()
                {
                    self.add_todo();
                }
            });

            ui.separator();

            // Stats
            let total_todos = self.todos.len();
            let completed_todos = self.todos.iter().filter(|t| t.completed).count();
            let pending_todos = total_todos - completed_todos;

            ui.horizontal(|ui| {
                ui.label(format!("Total: {}", total_todos));
                ui.label(format!("Pending: {}", pending_todos));
                ui.label(format!("Completed: {}", completed_todos));

                if completed_todos > 0 {
                    if ui.button("Clear Completed").clicked() {
                        self.clear_completed();
                    }
                }
            });

            ui.separator();

            // Todo list
            egui::ScrollArea::vertical().show(ui, |ui| {
                if self.todos.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        ui.label("No todos yet. Add one above!");
                    });
                } else {
                    // Sort todos: incomplete first, then completed
                    let mut sorted_todos = self.todos.clone();
                    sorted_todos.sort_by_key(|todo| todo.completed);

                    for todo in &sorted_todos {
                        ui.horizontal(|ui| {
                            let checkbox_response = ui.checkbox(&mut todo.completed.clone(), "");
                            if checkbox_response.clicked() {
                                self.toggle_todo(todo.id);
                            }

                            let text_color = if todo.completed {
                                ui.style().visuals.weak_text_color()
                            } else {
                                ui.style().visuals.text_color()
                            };

                            ui.colored_label(text_color, &todo.text);

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.button("🗑").on_hover_text("Delete").clicked() {
                                        self.delete_todo(todo.id);
                                    }
                                },
                            );
                        });
                        ui.separator();
                    }
                }
            });
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        self.save_todos();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 600.0])
            .with_min_inner_size([300.0, 400.0])
            .with_title("Todo List"),
        ..Default::default()
    };

    eframe::run_native(
        "Todo List",
        options,
        Box::new(|_cc| Box::new(TodoApp::new())),
    )
}
