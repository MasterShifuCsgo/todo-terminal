mod pages;
mod wrapper;

use wrapper::Pages;

fn main() {
    let page = Pages::MenuPage(pages::menu_page::Menu {});
    page.render();
}
