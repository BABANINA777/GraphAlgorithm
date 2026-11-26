mod graph;
use graph::*;
use eframe::{egui};
use eframe::egui::Pos2;
use std::f64::consts::PI;
use eframe::egui::accesskit::NodeId;
use crate::Menu::{ContextMenu, NodeMenu};

fn main() -> eframe::Result  {// возвращает результат работы программы в рантайм
    //Блок 1. создание и заполнение графа для теста
    let mut graph = Graph::default();

    graph.add_node(Vec::default());
    graph.add_node(Vec::default());
    let _ = graph.add_node_wh_id(1,Vec::default());
    let _ = graph.add_node_wh_id(2,Vec::default());
    graph.add_neighbors(1,vec![2]);
    let _ = graph.add_node_wh_id(3,Vec::default());
    graph.add_neighbors(3,vec![1,2]);

    println!("{graph:#?}");//вывод структуры в дебаг режиме

    // Создание gui
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])  // Начальная ширина и высота окна
            .with_min_inner_size([400.0, 300.0]) // Минимальный размер окна
            .with_title("GraphAlgorithm"),
        ..Default::default()
    };
    let graph_app = GraphApp::new(graph);

    eframe::run_native(
        "GraphAlgorithm",
        options,
        // Передает кучу-объект (Box): замыкание, которое владеет graph_app
        // и при вызове возвращает Ok(указатель в куче на сам объект graph_app)
        Box::new(|_cc| Ok(Box::new(graph_app))),
    )// функция запускает gui рантайм
}

//данные переданющиеся в рантайм
struct GraphApp {
    graph: Graph,
    scene_rect: egui::Rect,
    node_positions: Vec<Pos2>,
    context_menu_pos: Option<Menu>,
    input_window: InputWindow,
    selected_node_positions: Vec<Pos2>,
}

//реадизация трейта чтобы можно было дозаполнять графа по функции
impl Default for GraphApp {
    fn default() -> Self {
        Self{
            scene_rect: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(2000.0, 2000.0)),
            graph: Graph::default(),
            node_positions: Vec::new(),
            context_menu_pos: None,
            input_window: InputWindow::default(),
            selected_node_positions: Vec::new(),
        }
    }
}
impl GraphApp {
    //при создании хранит координаты на холсте все точек добавленных в граф
    fn new(graph: Graph) -> Self {
        let scene_rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(2000.0, 2000.0));
        let mut app = GraphApp {
            scene_rect,
            graph,
            node_positions: Vec::new(),
            context_menu_pos: None,
            input_window: InputWindow::default(),
            selected_node_positions: Vec::new(),
        };
        let center = app.scene_rect.center();
        app.node_positions = app.calculate_cord(center);
        app
    }


    fn calculate_cord(&self, canvas_center: Pos2)->Vec<Pos2>{
        //формула коорд вершин из графа. первый круг по центру, запоминаем его координаты. расчитываем координаты кругов вокруг него, смотрим чтобы каждый был к другому не ближе 50px. метод возвращяет координаты всех кругов
        if self.graph.nodes.len() == 1{
            return vec![canvas_center];
        }
        else if self.graph.nodes.len() == 0{
            return vec![];
        }
        else {
            let mut cord_vec : Vec<Pos2> = vec![canvas_center];
            let nodes_count = self.graph.nodes.len();
            let mut remaining_count = self.graph.nodes.len() - 1;
            let d :f64 = 200.0;
            let mut r :f64 = 0.0;

            while remaining_count > 0 {
                r += d;
                let mut a_min : f64 = (2.0*(d / (2.0 * r)).asin());
                let mut n_max = ((2.0*PI)/a_min).floor() as usize;
                if remaining_count >= n_max{
                    let mut current_a :f64 = 0.0;
                    for i in 0..n_max{
                        let x : f64 = canvas_center.x as f64 + r*current_a.cos();
                        let y : f64 = canvas_center.y as f64 + r*current_a.sin();
                        cord_vec.push(Pos2::new(x as f32,y as f32));
                        current_a += a_min;
                    }
                    remaining_count -= n_max;
                }
                else if remaining_count < n_max{
                    let mut current_a :f64 = 0.0;
                    for i in 0..remaining_count{
                        let x : f64 = canvas_center.x as f64 + r*current_a.cos();
                        let y : f64 = canvas_center.y as f64 + r*current_a.sin();
                        cord_vec.push(Pos2::new(x as f32,y as f32));
                        current_a += a_min;
                    }
                    break
                }

            }

            cord_vec
        }
    }


    fn draw_nodes(&self, painter: &egui::Painter) {
        //отрисовка на холсте всех точек. Заходит в граф, берет первую точку рисует ее, пишет на ней ее номер, помечает отрисованную точку, потом смотрит на соседей и рисует линию до соседа, потом его точку. помечает отрисованного соседа. когда все соседи отрисованны идет к след точки, отрисовка всегда начинается с проверки не рисовался ли уже точка с данным номером
        //настройка дизайна вершин
        let radius = 25.0;
        let fill_color = egui::Color32::from_rgb(60, 120, 220);
        let border = egui::Stroke::new(2.0, egui::Color32::WHITE);

        // Пробегаемся по всем вершинам и рисуем линии до соседей:
        for (index, node) in self.graph.nodes.iter().enumerate() {
            let start_pos = self.node_positions[index];

            for node_neighbor in &node.neighbors{
                match self.graph.nodes.iter().position(|n| n.id == *node_neighbor) {
                    Some(neighbor_index) => {
                        let end_pos = self.node_positions[neighbor_index];
                        //рисуем стрелку связи
                        painter.line_segment([start_pos,end_pos],egui::Stroke::new(2.0, egui::Color32::BLACK));
                    }
                    None => {}
                }

            }
        }

        // ПРОХОД 2: ВЕРШИНЫ (поверх линий)
        for (index, node) in self.graph.nodes.iter().enumerate() {
            let pos = self.node_positions[index];
            // 1. Рисуем кружок:
            painter.circle_filled(pos, radius, fill_color);
            painter.circle_stroke(pos, radius, border);
            // 2. Рисуем текст с номером:
            let label = format!("{}", node.id);
            painter.text(
                pos,
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::proportional(16.0),
                egui::Color32::WHITE,
            );
        }
    }


    //вызывается каждую секунду и обрабатывает нажатие кликов по сцене
    fn handle_canvas_click(&mut self, ui: &mut egui::Ui, world_bounds : egui::Rect){
        let response = ui.allocate_rect(world_bounds, egui::Sense::click());
        if response.secondary_clicked() {
            let cursor_cord : Option<Pos2> = response.interact_pointer_pos();
            match cursor_cord {
                Some(cursor_cord_pos) => {
                    for (index, node_cord) in self.node_positions.iter().enumerate(){
                        if (cursor_cord_pos.distance(*node_cord))<=25.0 {  //если кликнули на вершину
                            self.context_menu_pos = Some(NodeMenu((cursor_cord_pos,index)));
                            return;
                        }
                        else if (cursor_cord_pos.distance(*node_cord))<50.0{//если кликнули рядом с вершиной
                            self.context_menu_pos = None;
                            return;
                        }
                        else if (cursor_cord_pos.distance(*node_cord))>=50.0 {
                            continue;
                        }
                    }
                    self.context_menu_pos = Some(ContextMenu((cursor_cord_pos)));

                }
                None => {return;}

            }
        }
        else if response.clicked() {
            self.context_menu_pos = None;
            let cursor_cord: Option<Pos2> = response.interact_pointer_pos();
            match cursor_cord {
                Some(cursor_cord_pos) => {
                    for (index, node_cord) in self.node_positions.iter().enumerate() {
                        if (cursor_cord_pos.distance(*node_cord)) <= 25.0 {  //если кликнули на вершину

                        }
                    }
                }
                None => {}
            }
        }
        return

    }


    fn draw_menu(&mut self, ui: &mut egui::Ui) {
        match self.context_menu_pos {
            Some(current_menu) => {
                match current_menu {
                    ContextMenu(world_pos) => {
                        let transform = ui.ctx().layer_transform_to_global(ui.layer_id());
                        let screen_pos = transform.map(|t| t * world_pos).unwrap_or(world_pos);
                        let menu_pos = screen_pos + egui::vec2(10.0, 0.0);
                        egui::Area::new(egui::Id::unique("canvas_context_menu"))
                            .fixed_pos(menu_pos)
                            .show(ui, |ui| {
                                // Frame::popup добавляет стандартный фон, рамку и тень меню
                                egui::Frame::popup(ui.style()).show(ui, |ui| {
                                    if ui.button("Добавить вершину").clicked() {
                                        //маленькое окошечко с вводом делаемого id, если id не ввели вызывается add_node
                                        self.input_window.new_node_id_input.clear();                 // Очищаем поле ввода от прошлых значений
                                        self.input_window.error_msg = None;
                                        self.input_window.focus_input = true;
                                        self.input_window.show_add_node_window = true;               // Открываем окошко
                                        self.context_menu_pos = None; // Закрываем меню после нажатия
                                        self.input_window.pending_node_pos = Some(world_pos);// передаем координаты клика
                                    }
                                });
                            });
                    }
                    NodeMenu(pos_and_id) => {
                        let transform = ui.ctx().layer_transform_to_global(ui.layer_id());
                        let screen_pos = transform.map(|t| t * pos_and_id.0).unwrap_or(pos_and_id.0);
                        let menu_pos = screen_pos + egui::vec2(10.0, 0.0);
                        egui::Area::new(egui::Id::unique("canvas_node_menu"))
                            .fixed_pos(menu_pos)
                            .show(ui, |ui| {
                                egui::Frame::popup(ui.style()).show(ui, |ui| {
                                    if ui.button("Удалить вершину").clicked() {
                                        if self.graph.del_node_by_index(pos_and_id.1).is_ok() {
                                            self.node_positions.remove(pos_and_id.1);
                                        }
                                        self.context_menu_pos = None; // Закрываем меню после нажатия
                                    }
                                });
                            });
                    }
                }


            }
            None => {}
        }
    }


    // рисует окошко ввода желаемого id при включенном флаге
    fn draw_input_window(&mut self, ui: &mut egui::Ui) {
        if !self.input_window.show_add_node_window {
            return;
        }

        // Создаем плавающее окошко по центру экрана
        egui::Window::new("Новая вершина")
            .collapsible(false) // Убираем кнопку сворачивания
            .resizable(false)   // Запрещаем менять размер
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO) // Центрируем
            .show(ui, |ui| {
                ui.label("Введите ID (оставьте пустым для авто-ID):");

                // Поле ввода текста
                let response = ui.text_edit_singleline(&mut self.input_window.new_node_id_input);

                // Запрашиваем фокус только при открытии окна, а не каждый кадр.
                if self.input_window.focus_input {
                    response.request_focus();
                    self.input_window.focus_input = false;
                }

                // Проверяем: нажал ли пользователь кнопку "Создать" ИЛИ клавишу Enter в поле ввода
                let enter_pressed = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                ui.horizontal(|ui| {
                    if ui.button("Создать").clicked() || enter_pressed {
                        let trimmed = self.input_window.new_node_id_input.trim();

                        let add_result = if trimmed.is_empty() {
                            // Если поле пустое -> вызываем обычный add_node
                            self.graph.add_node(Vec::default()); // добавляем в граф
                            Ok(())
                        } else if let Ok(custom_id) = trimmed.parse::<i32>() {
                            // Если ввели корректное число -> вызываем add_node_wh_id
                            self.graph.add_node_wh_id(custom_id, Vec::default())
                        } else {
                            Err("ID должен быть целым числом".to_string())
                        };

                        if let Err(error) = add_result {
                            self.input_window.error_msg = Some(error);
                        } else {
                            if let Some(pos) = self.input_window.pending_node_pos {
                                self.node_positions.push(pos);
                            }
                            self.input_window.show_add_node_window = false;
                            self.input_window.pending_node_pos = None;
                        }
                    }

                    if ui.button("Отмена").clicked() {
                        self.input_window.show_add_node_window = false;
                        self.input_window.pending_node_pos = None;
                    }
                });
                if let Some(error) = &self.input_window.error_msg {
                    ui.colored_label(egui::Color32::RED, error);
                }
            });
    }


    //рисует круг вокруг выбранных коорд
    fn draw_node_state(&self,painter: &egui::Painter) {
        //TODO смотрит в self.selected_node_positions и рисует круг толщиной 5 вокруг координат?
    }
}

impl eframe::App for GraphApp {
    // Этот метод вызывается компилятором КАЖДЫЙ КАДР (60 раз в секунду):
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let left = egui::Panel::left("left_panel").exact_size(60.0);
        left.show(ui, |ui| { ui.horizontal(|ui| {}); });

        let bottom = egui::Panel::bottom("bottom_panel").exact_size(60.0);
        bottom.show(ui, |ui| {
            ui.horizontal(|ui| {

                let btn_load = egui::Button::new("Загрузить cvs");
                let response_load = ui.add_sized([120.0, 40.0], btn_load);
                if response_load.clicked() {

                }

                let btn_save = egui::Button::new("Cохранить в cvs");
                let response_save = ui.add_sized([120.0, 40.0], btn_save);


            });
        });


        let central = egui::CentralPanel::default().frame(egui::Frame::new().fill(egui::Color32::WHITE));
        let canvas_center = self.scene_rect.center();
        central.show(ui, |ui| { // передаем в show текущий ui и замыкание (структуру), принимающее новый ui.
            // Неявно компилятор создает структуру, которая хранит ссылку &mut self на наш GraphApp,
            // и реализует для неё трейт FnOnce с кодом из фигурных скобок.
            // В метод show передается сам экземпляр этой структуры по значению.


            //Блок 1. создаем сцену и определяем ее возможности в зависимости от зажатого Ctrl
            let is_ctrl_pressed = ui.input(|i| i.modifiers.command || i.modifiers.ctrl);//Проверяем, зажат ли сейчас Ctrl
            let scene = egui::Scene::new()// создаем сцену и передаем в нее структуры, если ctrl зажат то она позволит двигать сцену
                .zoom_range(0.1..=5.0)
                .sense(if is_ctrl_pressed {
                    egui::Sense::drag()
                } else {
                    egui::Sense::hover()
                });

            //Блок 2. делаем копию Rect, рисуем на ней.
            let mut scene_rect = self.scene_rect;

            scene.show(ui, &mut scene_rect, |ui| {
                // Внутри этого замыкания ui работает уже в МЕСТНЫХ координатах сцены!
                let painter = ui.painter();


                //рисуем рамку границ всего нашего мира (2000x2000)
                let world_bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(2000.0, 2000.0));
                painter.rect_stroke(
                    world_bounds,
                    0.0,
                    egui::Stroke::new(2.0, egui::Color32::LIGHT_GRAY),
                    egui::StrokeKind::Inside,
                );
                self.draw_nodes(painter);
                self.handle_canvas_click(ui, world_bounds);
                self.draw_menu(ui);

            });
            self.scene_rect = scene_rect;//Возвращяем копию в состояние приложения
            self.draw_input_window(ui);
        });
    }

}

#[derive(Clone, Copy)]
enum Menu{
    ContextMenu(eframe::egui::Pos2),
    NodeMenu((eframe::egui::Pos2, usize)),
}
struct InputWindow{
    show_add_node_window: bool,        // Флаг: открыто ли окошко ввода ID
    new_node_id_input: String,         // Буфер для текста, который вводит пользователь
    pending_node_pos: Option<Pos2>,    // Запоминаем координаты клика в мире графа
    error_msg: Option<String>,
    focus_input: bool,
}
impl Default for InputWindow{
    fn default()->Self{
        let mut i_w = InputWindow{
            show_add_node_window: false,
            new_node_id_input: "".to_string(),
            pending_node_pos: None,
            error_msg: None,
            focus_input: false,
        };
        i_w
    }
}



