var menu_visible = false;
window.onload = function () {
    const bars_menu = document.getElementById("bars");
    const menu_wrapper_wrapper = document.getElementById("menu-wrapper-wrapper");
    bars_menu.onclick = function () {
        if (menu_visible) {
            menu_wrapper_wrapper.style.right = "-210px";
            menu_visible = false;
        } else {
            menu_wrapper_wrapper.style.right = "0";
            menu_visible = true;
        }
    }
};