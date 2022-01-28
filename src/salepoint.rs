#![allow(unused_variables,unused_imports,dead_code)]

use std::path::Path;

use eframe::{epi::App, egui::{self, CtxRef, TopBottomPanel, Layout, Button, RichText, Separator, Label, Color32, Sense, FontDefinitions, TextStyle, FontFamily, SidePanel, panel::Side, CentralPanel, Ui, ScrollArea, Window}};
use edsa_pos::{pipeline::{
    accounts::{Debtor, Creditor, OutTransaction, TransactionIn}, 
    inventory::{FinishedProd, RawMaterial, PackagedProd, Product, Production}, 
    people::{Person, Employee}
}, fetch_logs, PathOption, LogPartial};

use crate::styles::top_panel_frame;


/*********Setting Up***********/
pub struct DailyRecords;
#[derive(Clone)]
pub struct TempVecs {
    rm: Vec<RawMaterial>,
    item_list: Vec<String>,
    actual_item_list: Vec<RawMaterial>,
    pip: Vec<Person>,
    pip_actual: Vec<Person>,
    pkg: Vec<PackagedProd>,
    actual_pkg_list: Vec<PackagedProd>,
}
impl Default for TempVecs {
    fn default() -> Self {
        let pip_actual: Vec<Person> = Vec::new();
        Self { 
            rm: Default::default(), 
            item_list: Default::default(), 
            actual_item_list: Default::default(), 
            pip: Default::default(), 
            pip_actual,
            pkg: Default::default(),
            actual_pkg_list: Default::default(),
        }
    }
}

pub struct PkgLists {
    money_in: Vec<TransactionIn>,
    money_out: Vec<OutTransaction>,
    staff: Vec<Employee>,
    people: Vec<Person>,
    product: Vec<Product>,
    production: Vec<Production>,
    fin_prod: Vec<FinishedProd>,
    pkg_prod: Vec<PackagedProd>,
    raw_mat: Vec<RawMaterial>,
    debtors: Vec<Debtor>,
    creditors: Vec<Creditor>,
    // dyield: DailyRecords,    
}
impl Default for PkgLists {
    fn default() -> Self {
        let money_in = fetch_logs::<TransactionIn>(PathOption::TransIn).unwrap();
        let money_out = fetch_logs::<OutTransaction>(PathOption::TransOut).unwrap();
        let staff = fetch_logs::<Employee>(PathOption::Staff).unwrap();
        let people = fetch_logs::<Person>(PathOption::People).unwrap();
        let fin_prod = fetch_logs::<FinishedProd>(PathOption::FinProd).unwrap();
        let product = fetch_logs::<Product>(PathOption::Product).unwrap();
        let production = fetch_logs::<Production>(PathOption::Production).unwrap();
        let pkg_prod = fetch_logs::<PackagedProd>(PathOption::PkgProd).unwrap();
        let raw_mat = fetch_logs::<RawMaterial>(PathOption::RawMat).unwrap();
        let debtors = fetch_logs::<Debtor>(PathOption::Debtors).unwrap();
        let creditors = fetch_logs::<Creditor>(PathOption::Creditor).unwrap();

        Self { money_in, money_out, staff, people, fin_prod, product, production, pkg_prod, raw_mat, debtors, creditors, /*dyield*/ }
    }
}

pub struct Editor {
    a: String,
    b: String,
    c: String,
    d: String,
    e: String,
    f: String,
    g: String,
    h: String,
    i: String,
    j: String,
    k: bool,
    l: bool,
    m: bool,
    n: bool,
    o: bool,
}
impl Editor {
    pub fn new() -> Self {
        Self { 
            a:String::default(), 
            b:String::default(), 
            c:String::default(), 
            d:String::default(), 
            e:String::default(), 
            f:String::default(), 
            g:String::from("0"), 
            h:String::from("0"), 
            i:String::from("0"), 
            j:String::from("0"),
            k:false,
            l:false,
            m:false,
            n:false,
            o:false, 
        } 
    }
}
impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}


#[derive(Default)]
pub struct Config {
    win_config: WindowConfig,
    sale_config: SaleConfig,
    sale_normal_config: SaleNormalConfig,
    misc_pops: MiscPopWins,
}

#[derive(Default)]
pub struct WindowConfig {
    sales_win: bool,
    inventory_win: bool,
}
#[derive(Default)]
pub struct SaleConfig {
    normal_win: bool,
    debt_win: bool,
}
#[derive(Default)]
pub struct SaleNormalConfig {
    buy_win: bool,
    sell_win: bool,
}

pub struct MiscPopWins {
    edit_rawmat: bool,
    edit_pkgprod: bool, 
}
impl Default for MiscPopWins {
    fn default() -> Self {
        Self { edit_rawmat: false, edit_pkgprod: false }
    }
}



/************START**************/

pub struct State {
    apk: PkgLists,
    conf: Config,
    editor: Editor,
    tvecs: TempVecs,
}

impl Default for State {
    fn default() -> Self {
        Self { apk: PkgLists::default(), conf: Default::default(), editor: Editor::default(), tvecs: TempVecs::default() }
    }
}

impl App for State {
    fn setup(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame, _storage: Option<&dyn eframe::epi::Storage>) {
        // will configure fonts here
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame) {
        self.render_top_panel(ctx);
        // if self.conf.misc_pops.edit_rawmat {
        //     self.edit_rawmat(ctx);
        // }else {
        //     self.render_sales_win(ctx);
        // }
        self.render_inventory_win(ctx);
    }

    fn name(&self) -> &str {
        "Edsa Feeds"
    }
}

impl State {
    pub fn render_top_panel(&mut self, ctx: &CtxRef) {
        // let fnt = crate::styles::font_def();
        // ctx.set_fonts(fnt);
        let frame = top_panel_frame();

        TopBottomPanel::top("header").frame(frame)
         .show(ctx, |ui|{

            ui.set_style(crate::styles::top_panel_style());

            ui.add_space(5.);
            egui::menu::bar(ui, |ui|{
                ui.with_layout(Layout::left_to_right(), |ui|{
                    let inv = ui.add(Button::new( RichText::new("Inventory")
                      .strong().monospace().heading() ));
                    if inv.clicked(){
                        println!("inventory");
                    }
                    ui.add(Separator::default());

                    let cash = ui.add(Button::new(RichText::new("Cash")
                      .strong().monospace().heading().heading() ));
                    if cash.clicked(){
                        println!("MoneyMan");
                    }
                    ui.add(Separator::default());

                    let staff = ui.add(Button::new(RichText::new("Staff")
                      .strong().monospace().heading() ));
                    if staff.clicked(){
                        println!("Bees");
                    }   
                    ui.add(Separator::default());

                    let pips = ui.add(Button::new(RichText::new("Clientele")
                      .strong().monospace().heading() ));
                    if pips.clicked(){
                        println!("RainMen");
                    }
                    ui.add(Separator::default());
                });
                ui.with_layout(Layout::right_to_left(),|ui|{
                    ui.add_space(10.);
                    let theme_btn=ui.add(Button::new(RichText::new("🔆"))) ;
                    if theme_btn.clicked() {}
                });
            });
            ui.add_space(5.);
        });
    }  

    pub fn buy_window(&mut self, ui: &mut Ui, ctx: &CtxRef) {
               
        ui.columns(3, |col| {
            col[2].visuals_mut().override_text_color = Some(Color32::from_rgb(0, 164, 188));
            col[0].visuals_mut().override_text_color = Some(Color32::from_rgb(0, 164, 188));
            

            col[1].label(RichText::new("Main Window"));
            col[1].separator();
            col[1].horizontal(|ui|{
                ui.label(RichText::new("supplier").color(Color32::BLACK)); ui.add_space(15.);
                let choose = ui.button("choose"); ui.add_space(5.);
                if choose.clicked() {
                    self.editor.k = false; 
                    self.editor.l = true;
                }
                let add = ui.button("add new ➕"); ui.add_space(5.);
                if add.clicked() {
                    self.editor.l = false;
                    self.editor.k = true;
                }
            });
            col[1].add_space(10.);

            if !self.tvecs.pip_actual.is_empty() {
                col[1].label(RichText::new(&self.tvecs.pip_actual[0].name));
                let tel = format!("☎: {}",&self.tvecs.pip_actual[0].tel);
                col[1].label(RichText::new(&tel));
                col[1].separator();
            }

            col[1].horizontal(|ui|{
                ui.label(RichText::new("items' list").color(Color32::BLACK));
                ui.add_space(15.);
                if ui.button(RichText::new("add item ➕")).clicked() {
                    self.editor.n = true;
                };
            });
            col[1].add_space(10.);
            // col[1].separator();
            ScrollArea::vertical().id_source("raw_scroll").max_height(280.)
              .show(&mut col[1], |ui|{

                let mut index: usize = 0; 
                for (i, item) in self.tvecs.actual_item_list.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&item.name));
                        ui.separator();
                        if ui.button(RichText::new("❎")).clicked() {
                            index = i;
                            self.editor.o = true;
                        }
                    });
                    ui.add_space(5.);
                    ui.horizontal(|ui|{
                        let qty = format!("quantity: {}kg",item.quantity);
                        let price = format!("price: Ksh.{}",item.price_per.unwrap());
                        ui.label(RichText::new(&qty));
                        ui.separator();
                        ui.label(RichText::new(&price));
                    });
                    ui.add_space(5.);
                    ui.separator();
                }
                if self.editor.o {
                    self.tvecs.actual_item_list.remove(index);
                    self.editor.o = false;
                }
            });
            let total_cost: f32 = self.tvecs.actual_item_list.iter()
                .map(|item| item.quantity * item.price_per.unwrap()).sum();
            let tc = format!("total cost: {}",total_cost);
            col[1].add_space(15.);
            col[1].label(RichText::new(tc.to_string()));

            col[1].horizontal(|ui| {
                ui.label(RichText::new("Settle Bill: "));
                ui.text_edit_singleline(&mut self.editor.j);
            });

            if col[1].button("complete purchase ✅").clicked() {
                if !self.tvecs.pip_actual.is_empty() && !self.tvecs.actual_item_list.is_empty() {
                    let per = self.tvecs.pip_actual[0].to_owned();
                    let mut tr = OutTransaction::new(per);
                    for rm in &self.tvecs.actual_item_list {
                        tr.add(rm.to_owned());
                    }
                    // settle bill and log
                    if let Ok(bs) = self.editor.j.parse::<f32>() {
                        tr.settle_bill(bs);
                        let path = Path::new("records/out_acc");
                        tr.log(path);
                        // reset the temp lists
                        self.tvecs.item_list = Vec::new();
                        self.tvecs.actual_item_list = Vec::new();
                        self.tvecs.pip = Vec::new();
                        self.tvecs.pip_actual = Vec::new();
                    }
                    dbg!(tr);
                }
            }

            col[1].add_space(10.);
            
            
            if self.editor.k {
                col[0].add_space(30.);
                col[0].horizontal(|ui|{
                    let back =ui.add(Button::new(RichText::new("◀ back")));
                    if back.clicked() {
                        self.editor.k = false;
                        self.editor.l = true;
                    }
                    ui.separator();
                    ui.label("Add Person");
                });
                col[0].label("name");
                col[0].add_space(5.);
                col[0].text_edit_singleline(&mut self.editor.e);
                col[0].add_space(10.);

                col[0].label("tel no.");
                col[0].add_space(5.);
                col[0].text_edit_singleline(&mut self.editor.f);
                col[0].add_space(10.);

                if col[0].button("add").clicked() {
                    if self.editor.e.len()>1 && self.editor.f.len()>1 {
                        let name = self.editor.e.clone();
                        let tel = self.editor.f.clone();

                        let p = Person::new(name, tel);
                        // add to current vector
                        self.apk.people.push(p.clone());
                        // log
                        let path = std::path::Path::new("records/people");
                        p.log(path);
                        // clean up
                        self.editor.e = String::default();
                        self.editor.f = String::default();
                        // self.editor.i = false;
                    }
                }
                col[0].add_space(10.);
                col[0].separator();
            }
            
            if self.editor.l {
                col[0].add_space(30.);
                col[0].horizontal(|ui|{
                    let back =ui.add(Button::new(RichText::new("◀ back")));
                    if back.clicked() {
                        self.editor.k = true;
                        self.editor.l = false;
                    }
                    ui.separator();
                    ui.label("Search").on_hover_text("type to narrow search");
                });
                col[0].add_space(5.);
                let search = col[0].text_edit_singleline(&mut self.editor.f);
                col[0].add_space(5.);
                col[0].separator();

                if search.changed() {
                    let p: Vec<_> = self.apk.people.to_owned().into_iter().filter(|per|{
                        per.name.contains(&self.editor.f)
                    }).collect();
                    self.tvecs.pip = p;
                }

                ScrollArea::vertical().show(&mut col[0], |ui|{
                    for  sp in self.tvecs.pip.iter(){
                        ui.label(&sp.name);
                        ui.add_space(5.);
                        ui.label(&sp.tel);
                        ui.add_space(5.);
                        if ui.button("pick").clicked(){
                            self.tvecs.pip_actual = Vec::new();
                            self.tvecs.pip_actual.push(sp.to_owned());
                        };
                        ui.separator();
                    }
                });
            }

            // add item
            if self.editor.n {
                col[2].add_space(30.);
                if col[2].button(RichText::new("add new")).clicked() {
                    println!("add new");
                    self.editor.m = true;
                    self.editor.n = false;
                }

                col[2].separator();
                col[2].horizontal( |ui| {
                    ui.label(RichText::new("search"));
                    let search = ui.text_edit_singleline(&mut self.editor.a);
                    if search.changed() {
                        let p: Vec<_> = self.apk.raw_mat.to_owned().into_iter().filter(|rm|{
                            rm.name.contains(&self.editor.a)
                        }).collect();
                        dbg!(&p);
                        self.tvecs.rm = p;
                    }
                });
                col[2].separator();
    
                ScrollArea::vertical().id_source("search_scroll")
                  .show(&mut col[2], |ui|{
                    for rm in self.tvecs.rm.iter() {
                        ui.label(RichText::new(&rm.name));
                        ui.add_space(5.);
                        let tstr = format!("remaining quantity: {}",&rm.quantity);
                        ui.label(RichText::new(tstr));
                        ui.add_space(5.);
                        if ui.button("pick").clicked() {
                            // add to list // pop a window
                            self.tvecs.item_list.push(rm.name.to_owned());
                            dbg!(&self.tvecs.item_list);
                            self.conf.misc_pops.edit_rawmat = true;
                        }
                        ui.add_space(5.);
                        ui.separator();
                    }
                });
            }
            // add new raw material
            if self.editor.m {
                col[2].add_space(30.);
                col[2].label(RichText::new("type name below.."));
                col[2].add_space(7.);
                col[2].text_edit_singleline(&mut self.editor.b);
                col[2].add_space(7.);
                col[2].label(RichText::new("quantity in store already"));
                col[2].text_edit_singleline(&mut self.editor.g);
                
                if col[2].button(RichText::new("add")).clicked() {
                    if let Ok(g) = self.editor.g.parse::<u32>() {
                        if self.editor.b.len() > 1 {
                            RawMaterial::new( self.editor.b.to_owned(), g as f32 ).local_log().unwrap();
                            self.apk.raw_mat = fetch_logs::<RawMaterial>(PathOption::RawMat).unwrap();
                            self.editor.b = String::default();
                            self.editor.g = 0.to_string();
                            self.editor.m = false;
                        }
                    }
                }
            }
        });   
    }

    pub fn edit_rawmat(&mut self, ctx: &CtxRef) {
        let frame = top_panel_frame();
        Window::new("edit").min_width(400.).frame(frame)
        .show(ctx, |ui|{
            let i = self.tvecs.item_list.len()-1;
            let name = &self.tvecs.item_list[i];
            ui.label(RichText::new(&*name));
            ui.horizontal(|ui|{
                ui.label(RichText::new("quantity:      "));
                ui.text_edit_singleline(&mut self.editor.h);
            });
            ui.horizontal(|ui|{
                ui.label(RichText::new("price offered:"));
                ui.text_edit_singleline(&mut self.editor.i);
            });
            ui.horizontal(|ui|{
                ui.add_space(300.);
                if ui.button(RichText::new("confirm")).clicked() {
                    if let Ok(qty) = self.editor.h.parse::<f32>() {
                        if let Ok(price) = self.editor.i.parse::<u32>() {
                            let mut item = RawMaterial::new(name.to_owned(), qty);
                            item.price(price);
                            self.tvecs.actual_item_list.push(item);
                            // and then the finisher
                            self.conf.misc_pops.edit_rawmat = false;
                        }
                    }
                }
                ui.add_space(5.);
                if ui.button(RichText::new("close")).clicked() {
                    self.conf.misc_pops.edit_rawmat = false;
                }
            });
        });
    }

    pub fn sell_window(&mut self, ui: &mut Ui) {
               
        ui.columns(3, |col| {
            col[2].visuals_mut().override_text_color = Some(Color32::from_rgb(0, 164, 188));
            col[0].visuals_mut().override_text_color = Some(Color32::from_rgb(0, 164, 188));
            

            col[1].label(RichText::new("Main Window"));
            col[1].separator();
            col[1].horizontal(|ui|{
                ui.label(RichText::new("Buyer").color(Color32::BLACK)); ui.add_space(15.);
                let choose = ui.button("choose"); ui.add_space(5.);
                if choose.clicked() {
                    self.editor.k = false; 
                    self.editor.l = true;
                }
                let add = ui.button("add new ➕"); ui.add_space(5.);
                if add.clicked() {
                    self.editor.l = false;
                    self.editor.k = true;
                }
            });
            col[1].add_space(10.);

            if !self.tvecs.pip_actual.is_empty() {
                col[1].label(RichText::new(&self.tvecs.pip_actual[0].name));
                let tel = format!("☎: {}",&self.tvecs.pip_actual[0].tel);
                col[1].label(RichText::new(&tel));
                col[1].separator();
            }

            col[1].horizontal(|ui|{
                ui.label(RichText::new("items' list").color(Color32::BLACK));
                ui.add_space(15.);
                if ui.button(RichText::new("add item ➕")).clicked() {
                    self.editor.n = true;
                };
            });
            col[1].add_space(10.);
            // col[1].separator();
            ScrollArea::vertical().id_source("raw_scroll").max_height(280.)
              .show(&mut col[1], |ui|{

                let mut index: usize = 0; 
                for (i, item) in self.tvecs.actual_pkg_list.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&item.pkg_specify));
                        let prinfo = format!("{}, {}kg",item.product.name, item.quantity);
                        ui.label(RichText::new(prinfo).strong().small());
                        ui.separator();
                        if ui.button(RichText::new("❎")).clicked() {
                            index = i;
                            self.editor.o = true;
                        }
                    });
                    ui.add_space(5.);
                    ui.horizontal(|ui|{
                        let qty = format!("amount: {} packs",item.quantity);
                        let price = format!("price: Ksh.{} each",item.cost);
                        ui.label(RichText::new(&qty));
                        ui.separator();
                        ui.label(RichText::new(&price));
                    });
                    ui.add_space(5.);
                    ui.separator();
                }
                if self.editor.o {
                    self.tvecs.actual_pkg_list.remove(index);
                    self.editor.o = false;
                }
            });
            let total_cost: u32 = self.tvecs.actual_pkg_list.iter()
                .map(|item| item.total * item.cost as u32).sum();
            let tc = format!("total cost: {}",total_cost);
            col[1].add_space(15.);
            col[1].label(RichText::new(tc.to_string()));

            col[1].horizontal(|ui| {
                ui.label(RichText::new("Settle Bill: "));
                ui.text_edit_singleline(&mut self.editor.j);
            });

            if col[1].button("complete purchase ✅").clicked() {
                if !self.tvecs.pip_actual.is_empty() && !self.tvecs.actual_item_list.is_empty() {

                    let per = self.tvecs.pip_actual[0].to_owned();
                    let mut tr = TransactionIn::new(per);
                    
                    for pkg in &self.tvecs.actual_pkg_list {
                        tr.add(pkg.to_owned());
                    }
                    // balance book, settle bill and log
                    if let Ok(bs) = self.editor.j.parse::<f32>() {
                        tr.balance_books();
                        tr.settle_bill(bs);
                        let path = Path::new("records/in_acc");
                        tr.log(path);
                        dbg!(&tr);
                        self.apk.money_in.push(tr);
                        // more clean up
                        self.apk.pkg_prod = fetch_logs::<PackagedProd>(PathOption::PkgProd).unwrap();
                        // reset the temp lists
                        self.tvecs.item_list = Vec::new();
                        self.tvecs.actual_pkg_list = Vec::new();
                        self.tvecs.pip = Vec::new();
                        self.tvecs.pip_actual = Vec::new();
                    }
                }
            }

            col[1].add_space(10.);
            
            
            if self.editor.k {
                col[0].add_space(30.);
                col[0].horizontal(|ui|{
                    let back =ui.add(Button::new(RichText::new("◀ back")));
                    if back.clicked() {
                        self.editor.k = false;
                        self.editor.l = true;
                    }
                    ui.separator();
                    ui.label("Add Person");
                });
                col[0].label("name");
                col[0].add_space(5.);
                col[0].text_edit_singleline(&mut self.editor.e);
                col[0].add_space(10.);

                col[0].label("tel no.");
                col[0].add_space(5.);
                col[0].text_edit_singleline(&mut self.editor.f);
                col[0].add_space(10.);

                if col[0].button("add").clicked() {
                    if self.editor.e.len()>1 && self.editor.f.len()>1 {
                        let name = self.editor.e.clone();
                        let tel = self.editor.f.clone();

                        let p = Person::new(name, tel);
                        // add to current vector
                        self.apk.people.push(p.clone());
                        // log
                        let path = std::path::Path::new("records/people");
                        p.log(path);
                        // clean up
                        self.editor.e = String::default();
                        self.editor.f = String::default();
                        // self.editor.i = false;
                    }
                }
                col[0].add_space(10.);
                col[0].separator();
            }
            
            if self.editor.l {
                col[0].add_space(30.);
                col[0].horizontal(|ui|{
                    let back =ui.add(Button::new(RichText::new("◀ back")));
                    if back.clicked() {
                        self.editor.k = true;
                        self.editor.l = false;
                    }
                    ui.separator();
                    ui.label("Search").on_hover_text("type to narrow search");
                });
                col[0].add_space(5.);
                let search = col[0].text_edit_singleline(&mut self.editor.f);
                col[0].add_space(5.);
                col[0].separator();

                if search.changed() {
                    let p: Vec<_> = self.apk.people.to_owned().into_iter().filter(|per|{
                        per.name.contains(&self.editor.f)
                    }).collect();
                    self.tvecs.pip = p;
                }
                
                ScrollArea::vertical().show(&mut col[0], |ui|{
                    for  sp in self.tvecs.pip.iter(){
                        ui.label(&sp.name);
                        ui.add_space(5.);
                        ui.label(&sp.tel);
                        ui.add_space(5.);
                        if ui.button("pick").clicked(){
                            self.tvecs.pip_actual = Vec::new();
                            self.tvecs.pip_actual.push(sp.to_owned());
                        };
                        ui.separator();
                    }
                });
            }

            // add item
            if self.editor.n {
                col[2].add_space(30.);
                if col[2].button(RichText::new("add new")).clicked() {
                    println!("add new");
                    self.editor.m = true;
                    self.editor.n = false;
                }

                col[2].separator();
                col[2].horizontal( |ui| {
                    ui.label(RichText::new("search"));
                    let search = ui.text_edit_singleline(&mut self.editor.a);
                    if search.changed() {
                        let p: Vec<_> = self.apk.pkg_prod.to_owned().into_iter().filter(|pkg|{
                            pkg.pkg_specify.contains(&self.editor.a)
                        }).collect();
                        dbg!(&p);
                        self.tvecs.pkg = p;
                    }
                });
                col[2].separator();
    
                ScrollArea::vertical().id_source("search_scroll")
                  .show(&mut col[2], |ui|{
                    for pkg in self.tvecs.pkg.iter() {
                        ui.label(RichText::new(&pkg.pkg_specify));
                        ui.add_space(5.);
                        let tstr = format!("in stock: {} packs",&pkg.total);
                        ui.label(RichText::new(tstr));
                        ui.add_space(5.);
                        if ui.button("pick").clicked() {
                            // add to list // pop a window
                            self.tvecs.item_list.push(pkg.pkg_specify.to_owned());
                            dbg!(&self.tvecs.item_list);
                            self.conf.misc_pops.edit_pkgprod = true;
                        }
                        ui.add_space(5.);
                        ui.separator();
                    }
                });
            }
            // *****start here********
            // add new packaged prod?
            if self.editor.m {
                col[2].add_space(30.);
                col[2].label(RichText::new("brand name.."));
                col[2].add_space(7.);
                col[2].text_edit_singleline(&mut self.editor.b);
                col[2].add_space(7.);
                col[2].label(RichText::new("quantity in store already"));
                col[2].text_edit_singleline(&mut self.editor.g);
                
                if col[2].button(RichText::new("add")).clicked() {
                    if let Ok(g) = self.editor.g.parse::<u32>() {
                        if self.editor.b.len() > 1 {
                            RawMaterial::new( self.editor.b.to_owned(), g as f32 ).local_log().unwrap();
                            self.apk.raw_mat = fetch_logs::<RawMaterial>(PathOption::RawMat).unwrap();
                            self.editor.b = String::default();
                            self.editor.g = 0.to_string();
                            self.editor.m = false;
                        }
                    }
                }
            }
        });
    }

    pub fn edit_pkgprod(&mut self, ctx: &CtxRef) {}

    pub fn render_inventory_win(&mut self, ctx: &CtxRef) {
        let frame = crate::styles::top_panel_frame();

        SidePanel::new(Side::Left, "left_side").min_width(300.).max_width(300.).frame(frame)
        .show(ctx, |ui| {
            // add a product
            ui.label(RichText::new("add a new product"));
            ui.text_edit_singleline(&mut self.editor.a);
            ui.add_space(5.);
            if ui.button("add").clicked() {
                
            }
            ui.separator();

            ui.label(RichText::new("raw materials").underline() );

            ScrollArea::vertical().id_source("raw_mat").max_height(100.)
            .show(ui, |ui| {
                for (i, rm) in self.apk.raw_mat.iter().enumerate() {
                    ui.horizontal(|ui|{
                        ui.label((i+1).to_string());
                        ui.separator();
                        ui.label(&rm.name);
                        ui.add_space(10.);
                        ui.label(&rm.quantity.to_string());
                    });
                }
            });

            ui.separator();

            ui.label(RichText::new("Finished Products"));

            ScrollArea::vertical().id_source("pkgprod").max_height(100.)
            .show(ui, |ui|{
                for (i, fp) in self.apk.fin_prod.iter().enumerate() {
                    ui.horizontal(|ui|{
                        ui.label(RichText::new((i+1).to_string()));
                        ui.separator();
                        ui.label(RichText::new(&fp.product.name));
                        ui.add_space(10.);
                        ui.label(RichText::new(&fp.quantity.to_string()));
                        ui.button(RichText::new("add"));
                    });
                }
            });
        });

        CentralPanel::default().frame(frame).show(ctx, |ui|{
            ui.label("main frame")
        });

    }

    pub fn render_sales_win(&mut self, ctx: &CtxRef){
        let frame = crate::styles::top_panel_frame();
        let sp = SidePanel::new(Side::Left, "side_menu").min_width(130.).max_width(130.).frame(frame);
        sp.show(ctx, |ui|{

            ui.set_style(crate::styles::top_panel_style());

            ui.add_space(10.);
            let trans = ui.add(Button::new(RichText::new("transactions")
              .strong().monospace() ));
            if trans.clicked(){
                self.conf.sale_config.normal_win = true;
                self.conf.sale_config.debt_win = false;
            }
            ui.add(Separator::default().spacing(10.) );
            let dt = ui.add(Button::new(RichText::new("debts")
              .strong().monospace() ));
            if dt.clicked(){
                self.conf.sale_config.normal_win = false;
                self.conf.sale_config.debt_win = true;
            }
            ui.add(Separator::default().spacing(10.) );

        });
        
        if self.conf.sale_config.normal_win {
            CentralPanel::default().frame(frame).show(ctx, |ui|{
                ui.add_space(15.);
                egui::menu::bar(ui, |ui|{

                    ui.set_style(crate::styles::top_panel_style());
          
                    let rt = RichText::new("buy");
                    let rst = RichText::new("sell");
                    let ls = ui.button(rst.heading()); 
                    ui.separator();
                    let lb = ui.button(rt.heading());
                    ui.separator();

                    if lb.clicked() {
                        self.conf.sale_normal_config.buy_win = true;
                        self.conf.sale_normal_config.sell_win = false;
                    }
                    if ls.clicked() {
                        self.conf.sale_normal_config.buy_win = false;
                        self.conf.sale_normal_config.sell_win = true;
                    }

                });
                if self.conf.sale_normal_config.buy_win {
                    ui.add_space(20.);
                    self.buy_window(ui, ctx);
                }else if self.conf.sale_normal_config.sell_win {
                    ui.add_space(20.);
                    self.sell_window(ui);
                }
                
            });

        }else if self.conf.sale_config.debt_win {
            CentralPanel::default().show(ctx, |ui|{
                // egui::menu::bar(ui, |ui|{
          
                //     let rt = RichText::new("External Debts").color(if self.config.ext_debts{ Color32::from_rgb(91,40,195) } else { Color32::GRAY });
                //     let rst = RichText::new("Internal Debts").color(if self.config.int_debts{ Color32::from_rgb(91,40,195) } else { Color32::GRAY });
                //     let lb = ui.button(rt.heading());
                //     ui.separator();
                //     let ls = ui.button(rst.heading()); 
                //     ui.separator();

                //     if lb.clicked() {
                //         self.config.ext_debts = true;
                //         self.config.int_debts = false;
                //     }
                //     if ls.clicked() {
                //         self.config.int_debts = true;
                //         self.config.ext_debts = false;
                //     }
                // });
                // if self.config.ext_debts {
                //     ui.add_space(20.);
                //     self.external_debts(ui);
                // }else if self.config.int_debts {
                //     ui.add_space(20.);
                //     self.internal_debts(ui);
                // }
            });
        }
    }

    pub fn configure_fonts(&self, ctx: &CtxRef){
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "broadway".to_owned(),
            egui::FontData::from_static(include_bytes!("/home/klan/edsa/edsafeeds/fonts/BroadwayRegular-7Bpow.ttf")),
        );
        font_def.family_and_size.insert(
            TextStyle::Body,
            (FontFamily::Monospace, 20.),
        );
        font_def.family_and_size.insert(
            TextStyle::Heading, 
            (FontFamily::Monospace, 30.),
        );
        font_def.fonts_for_family.get_mut(&FontFamily::Monospace)
         .unwrap()
         .insert(0, "broadway".to_owned());

        ctx.set_fonts(font_def);
    }

}