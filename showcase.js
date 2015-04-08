var last_slide;
var last_link;

function route(e) {
    if (e) e.preventDefault();
    if (old_hash) old_hash;
    if (location.hash) {
        if (last_slide) {
            last_slide.style.opacity = 0;
            last_slide.style["z-index"] = 1;
            last_link.style["background-color"] = "";
        }
        var without_hash = location.hash.substring(1, location.hash.length);
        var selector = ".showcase > article[id=\"" + without_hash + "\"]";
        var current_slide = document.querySelector(selector);
        current_slide.style.opacity = 1;
        current_slide.style["z-index"] = 10;
        last_slide = current_slide;
        selector = ".projectlist a[href=\"" + location.hash + "\"]";
        var current_link = document.querySelector(selector);
        current_link.style["background-color"] = "#155583";
        last_link = current_link;
    }
};


var old_hash = window.onhashchange;
window.onhashchange = route;

if (location.hash) {
    route()
} else {
    location.hash = "#Looptime";
}